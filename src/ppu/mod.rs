mod fetcher;
mod registers;
mod state_machine;
mod tile_maps;
mod tiles;

// https://gbdev.io/pandocs/Rendering.html
const FRAME_DOTS: u32 = 70224;
const SCANLINES_PER_FRAME: u32 = 154;
const T_CYCLES_PER_SCANLINE: u32 = FRAME_DOTS / SCANLINES_PER_FRAME;

const OAM_SCAN_DOTS: u32 = 80;
const PIXEL_DRAW_MIN_DOTS: u32 = 172;
const PIXEL_DRAW_MAX_T_CYCLES: u32 = 289;
const HBLANK_MIN_T_CYCLES: u32 = 87;
const HBLANK_MAX_DOTS: u32 = 204;
const VBLANK_DOTS: u32 = T_CYCLES_PER_SCANLINE * 10;

const HBLANK_MODE_NUMBER: u8 = 0;
const VBLANK_MODE_NUMBER: u8 = 1;
const OAM_SCAN_MODE_NUMBER: u8 = 2;
const PIXEL_DRAW_MODE_NUMBER: u8 = 3;

const WINDOW_WIDTH: u8 = 160;
const WINDOW_HEIGHT: u8 = 144;

use crate::{
    mmu::{self, memmap::*},
    util::{get_bit, set_bit},
};
use fetcher::{Fetcher, FetcherState};
use mmu::Mmu;
use std::{cell::RefCell, rc::Rc};

pub type GbBackground = [[u8; 256]; 256];
pub type GbDisplay = [[u8; 160]; 144];

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PpuMode {
    HBlank = HBLANK_MODE_NUMBER,
    VBlank = VBLANK_MODE_NUMBER,
    OamScan = OAM_SCAN_MODE_NUMBER,
    PixelDraw = PIXEL_DRAW_MODE_NUMBER,
}

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    was_enabled: bool,

    pub background: GbBackground,
    pub display: GbDisplay,

    fetcher: Fetcher,

    lx: u8,

    wy_triggered: bool,
    wy_counter: u8,
    wx_triggered: bool,
    window_drawn_this_scanline: bool,

    frame_dots: u32,
    scanline_dots: u32,
    mode_dots: u32,

    ly: u8,
    prev_stat_interrupt_signal: bool,
}

impl Ppu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Self {
        Ppu {
            mmu,
            was_enabled: false,

            background: [[0; 256]; 256],
            display: [[0; 160]; 144],

            fetcher: Fetcher::new(),

            lx: 0,

            wy_triggered: false,
            wy_counter: 0,
            wx_triggered: false,
            window_drawn_this_scanline: false,

            frame_dots: 0,
            scanline_dots: 0,
            mode_dots: 0,

            ly: 0,
            prev_stat_interrupt_signal: false,
        }
    }

    /// This function progresses the state of the PPU by one t-cycle.
    pub fn tick(&mut self) -> bool {
        let mut frame_complete = false;

        let ppu_mode = self.get_mode();
        let enabled = self.get_lcdc_flag(LCD_AND_PPU_ENABLE_BIT);

        // You're not supposed to turn off the PPU outside of vblank mode, but from
        // what I can tell, the hardware won't prevent it
        if self.was_enabled && !enabled {
            self.turn_off();
        }
        self.was_enabled = enabled;

        if !enabled {
            return frame_complete;
        }

        self.frame_dots += 1;
        self.scanline_dots += 1;
        self.mode_dots += 1;

        match ppu_mode {
            PpuMode::OamScan => self.oam_scan(),
            PpuMode::PixelDraw => self.pixel_draw(),
            PpuMode::HBlank => self.hblank(),
            PpuMode::VBlank => self.vblank(),
        }

        if self.scanline_dots == T_CYCLES_PER_SCANLINE {
            self.scanline_dots = 0;
            self.inc_ly();
        }

        if self.frame_dots == FRAME_DOTS {
            self.frame_dots = 0;
            self.reset_ly();
            frame_complete = true;
        }

        frame_complete
    }

    fn inc_ly(&mut self) {
        self.ly += 1;
        self.mmu.borrow_mut().write_byte_override(LY_ADDR, self.ly);

        self.update_wy();
        self.window_drawn_this_scanline = false;

        self.update_ppu_status_registers();
    }

    fn reset_ly(&mut self) {
        self.ly = 0;
        self.mmu.borrow_mut().write_byte_override(LY_ADDR, self.ly);

        self.wy_counter = 0;
        self.wy_triggered = false;

        self.update_ppu_status_registers();
    }

    // Check if the new scanline is in a window
    fn update_wy(&mut self) {
        let wy = self.read_byte(WY_ADDR);

        if self.window_drawn_this_scanline {
            self.wy_counter += 1;
        }

        if self.ly == wy {
            self.wy_triggered = self.ly == wy;
        }
    }

    fn update_ppu_status_registers(&mut self) {
        let ly = self.read_byte(LY_ADDR);

        // PPU mode is updated during state machine transitions, so it doesn't need to be done here.
        // But LY == LYC bit still needs to be updated
        let mut stat_byte = self.read_byte(STAT_ADDR);
        let lyc = self.read_byte(LYC_ADDR);
        set_bit(&mut stat_byte, LY_EQUALS_LYC_BIT, ly == lyc);
        self.mmu
            .borrow_mut()
            .write_byte_override(STAT_ADDR, stat_byte); // This byte is normally read-only

        // Status interrupt selects
        let enable_ly_equals_lyc = get_bit(stat_byte, LYC_INT_SELECT_BIT);
        let enable_hblank = get_bit(stat_byte, MODE_0_INT_SELECT_BIT);
        let enable_vblank = get_bit(stat_byte, MODE_1_INT_SELECT_BIT);
        let enable_oam = get_bit(stat_byte, MODE_2_INT_SELECT_BIT);

        // STAT interrupts are triggered on a rising edge in the stat_interrupt_line variable
        // Weird behavior with VBlank mode triggering with vblank select OR oam select described here:
        // https://raw.githubusercontent.com/geaz/emu-gameboy/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf
        // On page 29 section 8.7, STAT Interrupt
        let mode = self.get_mode();
        let stat_interrupt_signal = ((ly == lyc) && enable_ly_equals_lyc)
            || ((mode == PpuMode::HBlank) && enable_hblank)
            || ((mode == PpuMode::OamScan) && (enable_oam))
            || ((mode == PpuMode::VBlank) && (enable_vblank || enable_oam));

        if stat_interrupt_signal && !self.prev_stat_interrupt_signal {
            self.mmu.borrow_mut().request_interrupt(STAT_INTERRUPT_BIT);
        }
        self.prev_stat_interrupt_signal = stat_interrupt_signal;
    }

    pub fn get_mode(&mut self) -> PpuMode {
        // The mode is represented by the rightmost two bits of the LCDC register.
        let byte = self.read_byte(STAT_ADDR);
        let mode_number = byte & 0b_0000_0011;

        match mode_number {
            HBLANK_MODE_NUMBER => PpuMode::HBlank,
            VBLANK_MODE_NUMBER => PpuMode::VBlank,
            OAM_SCAN_MODE_NUMBER => PpuMode::OamScan,
            PIXEL_DRAW_MODE_NUMBER => PpuMode::PixelDraw,
            _ => unreachable!("Impossible value for ppu mode"),
        }
    }

    pub fn set_mode(&mut self, mode: PpuMode) {
        // Only the rightmost two bits should be touched
        let mode_number = mode as u8;
        let mut byte = self.read_byte(STAT_ADDR);
        byte &= 0b_1111_1100;
        byte |= mode_number;

        self.mmu.borrow_mut().write_byte_override(STAT_ADDR, byte);

        self.mode_dots = 0;
        self.update_ppu_status_registers();
    }

    fn turn_off(&mut self) {
        self.frame_dots = 0;
        self.scanline_dots = 0;
        self.mode_dots = 0;
        self.ly = 0;
        self.prev_stat_interrupt_signal = false;
        self.wy_counter = 0;
        self.wy_triggered = false;
        self.wx_triggered = false;
        self.set_mode(PpuMode::OamScan);

        let mut mmu = self.mmu.borrow_mut();
        mmu.write_byte_override(LY_ADDR, 0x00);
        mmu.vram_lock = false;
        mmu.oam_lock = false;
    }

    /// The PPU is not write-locked from VRAM or OAM
    fn read_byte(&self, addr: u16) -> u8 {
        let region = map_region(addr);
        match region {
            MemRegion::Vram | MemRegion::Oam => self.mmu.borrow().read_byte_override(addr),
            _ => self.mmu.borrow().read_byte(addr),
        }
    }
}

mod debug {
    use crate::mmu::memmap::BG_AND_WINDOW_ENABLE_BIT;

    use super::*;
    impl Ppu {
        // This is so I can just draw a tilemap to the screen without worrying about tick cycles.
        pub fn splat_tiles(&mut self) {
            let mut addr = 0x9800_u16; // <-- This is where one tilemap starts

            // let mut addr = 0x9C00_u16; <-- This is where the other tilemap starts

            let addressing_mode = self.get_lcdc_flag(BG_AND_WINDOW_ENABLE_BIT);

            // Tilemap is 32x32 tiles
            for tile_row in 0..32_usize {
                for tile_col in 0..32_usize {
                    let tile_index = self.mmu.borrow().read_byte_override(addr);
                    let tile = self.get_tile(tile_index);
                    addr += 1;
                    // tile is 8x8 pixels
                    for pixel_row in 0..8_usize {
                        for pixel_col in 0..8_usize {
                            self.background[pixel_row + tile_row * 8][pixel_col + tile_col * 8] =
                                tile[pixel_row][pixel_col];
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
