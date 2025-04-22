mod registers;
mod tile_maps;
mod tiles;

// Timings are an integral part of the PPU
// https://gbdev.io/pandocs/Rendering.html
const T_CYCLES_PER_FRAME: u32 = 70224;
const SCANLINES_PER_FRAME: u32 = 154;
const T_CYCLES_PER_SCANLINE: u32 = T_CYCLES_PER_FRAME / SCANLINES_PER_FRAME;

const OAM_SCAN_T_CYCLES: u32 = 80;
const PIXEL_DRAW_MIN_T_CYCLES: u32 = 172;
const PIXEL_DRAW_MAX_T_CYCLES: u32 = 289;
const HBLANK_MIN_T_CYCLES: u32 = 87;
const HBLANK_MAX_T_CYCLES: u32 = 204;
const VBLANK_T_CYCLES: u32 = T_CYCLES_PER_SCANLINE * 10;

const HBLANK_MODE_NUMBER: u8 = 0;
const VBLANK_MODE_NUMBER: u8 = 1;
const OAM_SCAN_MODE_NUMBER: u8 = 2;
const PIXEL_DRAW_MODE_NUMBER: u8 = 3;

use crate::{
    mmu::{self, memmap::*},
    util::{get_bit, set_bit},
};
use mmu::Mmu;
use std::{cell::RefCell, rc::Rc};

pub type GbDisplay = [[u8; 256]; 256];

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
    pub display: GbDisplay,

    frame_t_cycle_count: u32,
    scanline_t_cycle_count: u32,
    mode_t_cycle_count: u32,

    scanline_counter: u8,
    stat_interrupt_line: bool,
}

impl Ppu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Self {
        Ppu {
            mmu,
            was_enabled: false,
            display: [[0; 256]; 256],

            frame_t_cycle_count: 0,
            scanline_t_cycle_count: 0,
            mode_t_cycle_count: 0,
            scanline_counter: 0,
            stat_interrupt_line: false,
        }
    }

    pub fn tick(&mut self) {
        let ppu_mode = self.get_mode();
        let ppu_enabled = self.get_lcdc_flag(LCD_AND_PPU_ENABLE_BIT);

        // You're not supposed to turn off the PPU outside of vblank mode, but from
        // what I can tell, the hardware won't prevent it.
        if self.was_enabled && !ppu_enabled {
            self.turn_off();
        }
        self.was_enabled = ppu_enabled;

        if !ppu_enabled {
            return;
        }

        self.frame_t_cycle_count += 1;
        self.scanline_t_cycle_count += 1;
        self.mode_t_cycle_count += 1;

        self.mmu.borrow_mut().bypass_write_byte_ly(self.scanline_counter);


        match ppu_mode {
            PpuMode::OamScan => {
                // OAMSCAN -> PIXELDRAW
                if self.scanline_t_cycle_count == OAM_SCAN_T_CYCLES {
                    self.set_mode(PpuMode::PixelDraw);
                    self.mmu.borrow_mut().vram_lock = true;
                }
            }
            PpuMode::PixelDraw => {
                // PIXELDRAW -> HBLANK
                if self.scanline_t_cycle_count == OAM_SCAN_T_CYCLES + PIXEL_DRAW_MIN_T_CYCLES {
                    self.set_mode(PpuMode::HBlank);
                    self.mmu.borrow_mut().vram_lock = false;
                    self.mmu.borrow_mut().oam_lock = false;
                }
            }
            PpuMode::HBlank => {
                // HBLANK -> VBLANK
                if self.frame_t_cycle_count == T_CYCLES_PER_FRAME - VBLANK_T_CYCLES {
                    self.set_mode(PpuMode::VBlank);
                    self.mmu
                        .borrow_mut()
                        .request_interrupt(VBLANK_INTERRUPT_BIT);
                // HBLANK -> OAMSCAN
                } else if self.scanline_t_cycle_count
                    == OAM_SCAN_T_CYCLES + PIXEL_DRAW_MIN_T_CYCLES + HBLANK_MAX_T_CYCLES
                {
                    self.set_mode(PpuMode::OamScan);
                    self.mmu.borrow_mut().oam_lock = true;
                }
            }
            PpuMode::VBlank => {
                if self.frame_t_cycle_count == T_CYCLES_PER_FRAME {
                    // println!("VBLANK -> OAM");
                    self.set_mode(PpuMode::OamScan);
                    self.mmu.borrow_mut().oam_lock = true;
                }
            }
        }

        if self.scanline_t_cycle_count == T_CYCLES_PER_SCANLINE {
            self.scanline_t_cycle_count = 0;
            self.scanline_counter += 1;
        }

        if self.frame_t_cycle_count == T_CYCLES_PER_FRAME {
            self.frame_t_cycle_count = 0;
            self.scanline_counter = 0;
        }

        // LY and the LY=LYC bit of the STAT register are updated each cycle,
        // and interrupts are requested based on the current PPU mode and stat register.
        self.update_ppu_status_registers();
    }

   

    fn update_ppu_status_registers(&mut self) {
        let ly = self.read_byte(LY_ADDR);

        // PPU mode is updated during state machine transitions, so it doesn't need to be done here.
        // But LY == LYC bit still needs to be updated
        let mut stat_byte = self.read_byte(STAT_ADDR);
        let lyc = self.read_byte(LYC_ADDR);
        set_bit(&mut stat_byte, LY_EQUALS_LYC_BIT, ly == lyc);
        self.mmu.borrow_mut().bypass_write_byte_stat(stat_byte); // This byte is normally read-only

        // Statis interrupt selects
        let enable_ly_equals_lyc = get_bit(stat_byte, LYC_INT_SELECT_BIT);
        let enable_hblank = get_bit(stat_byte, MODE_0_INT_SELECT_BIT);
        let enable_vblank = get_bit(stat_byte, MODE_1_INT_SELECT_BIT);
        let enable_oam = get_bit(stat_byte, MODE_2_INT_SELECT_BIT);

        // STAT interrupts are triggered on a rising edge in the stat_interrupt_line variable
        // Weird behavior with VBlank mode triggering with vblank select OR oam select described here:
        // https://raw.githubusercontent.com/geaz/emu-gameboy/master/docs/The%20Cycle-Accurate%20Game%20Boy%20Docs.pdf
        // On page 29 section 8.7, STAT Interrupt
        let mode = self.get_mode();
        let stat_interrupt_line = ((ly == lyc) && enable_ly_equals_lyc)
            || ((mode == PpuMode::HBlank) && enable_hblank)
            || ((mode == PpuMode::OamScan) && (enable_oam))
            || ((mode == PpuMode::VBlank) && (enable_vblank || enable_oam));

        if stat_interrupt_line && !self.stat_interrupt_line {
            self.mmu.borrow_mut().request_interrupt(STAT_INTERRUPT_BIT);
        }
        self.stat_interrupt_line = stat_interrupt_line;
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
        let mut byte = self.read_byte(LCDC_ADDR);
        byte &= 0b_1111_1100;
        byte |= mode_number;

        self.mmu.borrow_mut().bypass_write_byte_stat(byte);
    }

    fn turn_off(&mut self) {
        self.frame_t_cycle_count = 0;
        self.scanline_t_cycle_count = 0;
        self.scanline_counter = 0;
        self.set_mode(PpuMode::OamScan);
        self.mmu.borrow_mut().bypass_write_byte_ly(0x00);
        self.mmu.borrow_mut().vram_lock = false;
        self.mmu.borrow_mut().oam_lock = false;
    }
    
    // The PPU is not affected by the write lock on VRAM - it always bypasses it
    fn read_byte(&self, addr: u16) -> u8 {
        self.mmu.borrow().bypass_read_byte_vram(addr)
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
            // println!("Addressing Mode: {}", addressing_mode);

            // Tilemap is 32x32 tiles
            for tile_row in 0..32_usize {
                for tile_col in 0..32_usize {
                    let tile_index = self.mmu.borrow().bypass_read_byte_vram(addr);
                    // println!("{:04x}: {:02x}", addr, tile_index);
                    let tile = self.get_tile(tile_index);
                    // let tile = self.get_tile_direct_index(0x8000 + tile_index as u16 * 16);
                    // let tile = self.get_test_tile(TEST_TILE_RAW);
                    addr += 1;
                    // tile is 8x8 pixels
                    for pixel_row in 0..8_usize {
                        for pixel_col in 0..8_usize {
                            self.display[pixel_row + tile_row * 8][pixel_col + tile_col * 8] =
                                tile[pixel_row][pixel_col];
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        create_gameboy_components, emulate_boot,
        util::get_bit,
    };

    // todo! This test is unfinished
    // Test PPU registers cycle-by-cycle
    // Comparing to timing table in the cycle-accurate gameboy docs, 8.9.1 - Timings in DMG
    // #[test]
    // fn test_ly_equals_lyc_interrupt_timing() {
    //     let (mmu, mut cpu, mut ppu) = create_gameboy_components();

    //     // The LCDC has to be on for anything to happen
    //     ppu.set_lcdc_flag(LCD_AND_PPU_ENABLE_BIT, true);

    //     let mut t_cycles: u32 = 0;

    //     loop {
    //         let stat = ppu.read_byte(STAT_ADDR);
    //         let stat_mode = ppu.get_mode() as u8;
    //         let lyc = ppu.read_byte(LYC_ADDR);

    //         let ly = ppu.read_byte(LY_ADDR);
    //         // let ly_to_compare_lyc = ly == lyc;
    //         let if_flag = get_bit(stat, LY_EQUALS_LYC_BIT) as u8;

    //         ppu.tick();

    //         match t_cycles {
    //             0..4 => {
    //                 test_t_cycle(t_cycles, ly, 0, stat_mode, 0, if_flag, 0);
    //             }
    //             4..448 => {
    //                 test_t_cycle(t_cycles, ly, 0, stat_mode, 2, if_flag, 0);
    //             }
    //             _ => break,
    //         }

    //         ppu.tick();
    //         t_cycles += 1;
    //     }
    // }

    // fn test_t_cycle(
    //     t_cycles: u32,
    //     ly: u8,
    //     ly_expect: u8,
    //     stat_mode: u8,
    //     stat_mode_expect: u8,
    //     if_flag: u8,
    //     if_flag_expect: u8,
    // ) {
    //     eprintln!("Cycle count: {}", t_cycles);
    //     assert_eq!(ly, ly_expect, "LY");
    //     assert_eq!(stat_mode, stat_mode_expect, "STAT Mode");
    //     // assert_eq!(ly_to_compare_lyc, false, "LY to compare LYC");
    //     assert_eq!(if_flag, if_flag_expect, "IF Flag (LY=LYC)");
    // }
}
