mod oam_scan;
mod pixel_draw;
mod state_machine;
mod tiles;

// The startup of the PPU is a bit buggy right now. It takes a cycle for
// everything to sync up properly

// https://gbdev.io/pandocs/Rendering.html
pub const FRAME_DOTS: u32 = 70224;
const SCANLINES_PER_FRAME: u32 = 154;
const DOTS_PER_SCANLINE: u32 = FRAME_DOTS / SCANLINES_PER_FRAME;

const OAM_SCAN_DOTS: u32 = 80;
const PIXEL_DRAW_MIN_DOTS: u32 = 172;
const PIXEL_DRAW_MAX_T_CYCLES: u32 = 289;
const HBLANK_MIN_T_CYCLES: u32 = 87;
const HBLANK_MAX_DOTS: u32 = 204;
const VBLANK_DOTS: u32 = DOTS_PER_SCANLINE * 10;

const HBLANK_MODE_NUMBER: u8 = 0;
const VBLANK_MODE_NUMBER: u8 = 1;
const OAM_SCAN_MODE_NUMBER: u8 = 2;
const PIXEL_DRAW_MODE_NUMBER: u8 = 3;

pub const DISPLAY_WIDTH: usize = 160;
pub const DISPLAY_HEIGHT: usize = 144;

use crate::{
    mmu::{self, memmap::*},
    ppu::oam_scan::OamData,
    util::{get_bit, set_bit},
};
use mmu::Mmu;
use pixel_draw::Fetcher;

pub type GbDisplay = [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PpuMode {
    HBlank = HBLANK_MODE_NUMBER,
    VBlank = VBLANK_MODE_NUMBER,
    OamScan = OAM_SCAN_MODE_NUMBER,
    PixelDraw = PIXEL_DRAW_MODE_NUMBER,
}

pub struct Ppu {
    was_enabled: bool,
    frame_complete: bool,

    pub display: GbDisplay,
    oam_data: OamData,

    fetcher: Fetcher,

    lx: u8,
    ly: u8,

    wy_triggered: bool,
    wy_counter: u8,
    wx_triggered: bool,
    window_drawn_this_scanline: bool,

    scanline_dots: u32,
    mode_dots: u32,

    prev_stat_interrupt_signal: bool,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            was_enabled: false,
            frame_complete: false,

            display: [[0; 160]; 144],
            oam_data: OamData::new(),

            fetcher: Fetcher::new(),

            lx: 0,

            wy_triggered: false,
            wy_counter: 0,
            wx_triggered: false,
            window_drawn_this_scanline: false,

            scanline_dots: 0,
            mode_dots: 0,

            ly: 0,
            prev_stat_interrupt_signal: false,
        }
    }

    /// This function progresses the state of the PPU by one t-cycle.
    pub fn tick(&mut self, mmu: &mut Mmu) -> bool {
        let ppu_mode = self.get_mode(mmu);
        let enabled = self.get_lcdc_flag(LCD_AND_PPU_ENABLE_BIT, mmu);

        // You're not supposed to turn off the PPU outside of vblank mode, but from
        // what I can tell, the hardware won't prevent it
        if self.was_enabled && !enabled {
            self.turn_off(mmu);
        }
        self.was_enabled = enabled;

        if !enabled {
            return false;
        }

        self.scanline_dots += 1;
        self.mode_dots += 1;

        match ppu_mode {
            PpuMode::OamScan => self.oam_scan(mmu),
            PpuMode::PixelDraw => self.pixel_draw(mmu),
            PpuMode::HBlank => self.hblank(mmu),
            PpuMode::VBlank => self.vblank(mmu),
        }

        let frame_complete = self.frame_complete;
        self.frame_complete = false;

        if frame_complete {
            self.overlay_object_display();
            self.clear_object_display();
        }

        frame_complete
    }

    fn inc_ly(&mut self, mmu: &mut Mmu) {
        self.scanline_dots = 0;
        self.ly += 1;
        self.lx = 0;
        mmu.write_byte_override(LY_ADDR, self.ly);

        self.update_wy(mmu);
        self.window_drawn_this_scanline = false;

        self.update_ppu_status_registers(mmu);
    }

    fn reset_ly(&mut self, mmu: &mut Mmu) {
        self.ly = 0;
        mmu.write_byte_override(LY_ADDR, self.ly);

        self.wy_counter = 0;
        self.wy_triggered = false;

        self.update_ppu_status_registers(mmu);
    }

    // Check if the new scanline is in a window
    /// WY is the y position at which a window begins.
    fn update_wy(&mut self, mmu: &mut Mmu) {
        let wy = self.read_byte(WY_ADDR, mmu);

        if self.window_drawn_this_scanline {
            self.wy_counter += 1;
        }

        if self.ly == wy {
            self.wy_triggered = self.ly == wy;
        }
    }

    /// STAT interrupts may occur either on line switches or mode changes.
    fn update_ppu_status_registers(&mut self, mmu: &mut Mmu) {
        let ly = self.read_byte(LY_ADDR, mmu);

        // PPU mode is updated during state machine transitions, so it doesn't need to be done here.
        // But LY == LYC bit still needs to be updated
        let mut stat_byte = self.read_byte(STAT_ADDR, mmu);
        let lyc = self.read_byte(LYC_ADDR, mmu);
        set_bit(&mut stat_byte, LY_EQUALS_LYC_BIT, ly == lyc);
        mmu.write_byte_override(STAT_ADDR, stat_byte); // This byte is normally read-only

        // Status interrupt selects
        let enable_ly_equals_lyc = get_bit(stat_byte, LYC_INT_SELECT_BIT);
        let enable_hblank = get_bit(stat_byte, MODE_0_INT_SELECT_BIT);
        let enable_vblank = get_bit(stat_byte, MODE_1_INT_SELECT_BIT);
        let enable_oam = get_bit(stat_byte, MODE_2_INT_SELECT_BIT);

        // STAT interrupts are triggered on a rising edge in the stat_interrupt_line variable
        // Weird behavior with VBlank mode triggering with vblank select OR oam select described here:
        // https://raw.githubusercontent.com/geaz/emu-gameboy/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf
        // On page 29 section 8.7, STAT Interrupt
        let mode = self.get_mode(mmu);
        let stat_interrupt_signal = ((ly == lyc) && enable_ly_equals_lyc)
            || ((mode == PpuMode::HBlank) && enable_hblank)
            || ((mode == PpuMode::OamScan) && (enable_oam))
            || ((mode == PpuMode::VBlank) && (enable_vblank || enable_oam));

        if stat_interrupt_signal && !self.prev_stat_interrupt_signal {
            mmu.request_interrupt(STAT_INTERRUPT_BIT);
        }
        self.prev_stat_interrupt_signal = stat_interrupt_signal;
    }

    pub fn get_mode(&mut self, mmu: &mut Mmu) -> PpuMode {
        let byte = self.read_byte(STAT_ADDR, mmu);
        let mode_number = byte & 0b_0000_0011;

        match mode_number {
            HBLANK_MODE_NUMBER => PpuMode::HBlank,
            VBLANK_MODE_NUMBER => PpuMode::VBlank,
            OAM_SCAN_MODE_NUMBER => PpuMode::OamScan,
            PIXEL_DRAW_MODE_NUMBER => PpuMode::PixelDraw,
            _ => unreachable!("Impossible value for ppu mode"),
        }
    }

    pub fn set_mode(&mut self, mode: PpuMode, mmu: &mut Mmu) {
        let mode_number = mode as u8;
        let mut byte = self.read_byte(STAT_ADDR, mmu);
        byte &= 0b_1111_1100;
        byte |= mode_number;

        mmu.write_byte_override(STAT_ADDR, byte);

        self.update_ppu_status_registers(mmu);
        self.mode_dots = 0;
    }

    fn turn_off(&mut self, mmu: &mut Mmu) {
        self.scanline_dots = 0;
        self.mode_dots = 0;
        self.ly = 0;
        self.prev_stat_interrupt_signal = false;
        self.wy_counter = 0;
        self.wy_triggered = false;
        self.wx_triggered = false;
        self.set_mode(PpuMode::OamScan, mmu);

        mmu.write_byte_override(LY_ADDR, 0x00);
        mmu.vram_lock = false;
        mmu.oam_lock = false;
    }

    /// The PPU is not write-locked from VRAM or OAM, so it gets a special write function
    fn read_byte(&self, addr: u16, mmu: &mut Mmu) -> u8 {
        let region = map_region(addr);
        match region {
            MemRegion::Vram | MemRegion::Oam => mmu.read_byte_override(addr),
            _ => mmu.read_byte(addr),
        }
    }

    pub fn get_lcdc_flag(&self, bit: u8, mmu: &mut Mmu) -> bool {
        let byte = self.read_byte(LCDC_ADDR, mmu);
        get_bit(byte, bit)
    }

    pub fn set_lcdc_flag(&mut self, bit: u8, set: bool, mmu: &mut Mmu) {
        let mut byte = self.read_byte(LCDC_ADDR, mmu);
        set_bit(&mut byte, bit, set);
        mmu.write_byte(LCDC_ADDR, byte);
    }

    fn overlay_object_display(&mut self) {
        for (y, row) in self.display.iter_mut().enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                let object_pixel = self.oam_data.object_display[y][x];
                if object_pixel != 0 {
                    *pixel = object_pixel;
                }
            }
        }
    }

    fn clear_object_display(&mut self) {
        for row in self.oam_data.object_display.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = 0;
            }
        }
    }
}
