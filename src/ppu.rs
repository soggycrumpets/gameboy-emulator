mod registers;
mod tile_maps;
mod tiles;

// Timings are an integral part of the PPU
// https://gbdev.io/pandocs/Rendering.html 
const T_CYCLES_PER_FRAME: u32 = 70224;
const SCANLINES_PER_FRAME: u32 = 154;
const T_CYCLES_PER_SCANLICE: u32 = T_CYCLES_PER_FRAME / SCANLINES_PER_FRAME;

const OAM_SCAN_T_CYCLES: u32 = 80;
const PIXEL_DRAW_MIN_T_CYCLES: u32 = 172;
const PIXEL_DRAW_MAX_T_CYCLES: u32 = 289;
const HBLANK_MIN_T_CYCLES: u32 = 87;
const HBLANK_MAX_T_CYCLES: u32 = 204;
const VBLANK_T_CYCLES: u32 = T_CYCLES_PER_SCANLICE * 10;


use crate::mmu;
use mmu::Mmu;
use registers::LcdControlFlag;
use std::{cell::RefCell, rc::Rc};
use tiles::Tile;

pub type GbDisplay = [[u8; 256]; 256];

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    pub display: GbDisplay,
}

impl Ppu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Self {
        Ppu {
            mmu,
            display: [[0; 256]; 256],
        }
    }

    fn tick(&mut self) {
        
        // Get tile ID
        // Get tile row (low)
        // Get tile row (high)
        // Push pixels
    }
}

mod debug {
    use super::*;
    impl Ppu {
        // This is so I can just draw a tilemap to the screen without worrying about tick cycles.
        pub fn splat_tiles(&mut self) {
            let mut addr = 0x9800_u16; // <-- This is where one tilemap starts

            // let mut addr = 0x9C00_u16; <-- This is where the other tilemap starts

            let addressing_mode = self.get_lcd_control_flag(LcdControlFlag::BgAndWindowEnable);
            // println!("Addressing Mode: {}", addressing_mode);

            // Tilemap is 32x32 tiles
            for tile_row in 0..32_usize {
                for tile_col in 0..32_usize {
                    let tile_index = self.mmu.borrow().read_byte(addr);
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
