mod registers;
mod tile_maps;
mod tiles;

use crate::mmu;
use mmu::Mmu;
use registers::{LcdControlFlag, LcdStatusFlag, Register};
use std::{cell::RefCell, rc::Rc};
use tiles::Tile;

pub type GbDisplay = [[u8; 256]; 256];

const TEST_TILE_RAW: [u8; 16] = [
    0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C,
];

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

    pub fn draw(&mut self) {
        // For now, just look through a whole tileset
        let mut addr = 0x9800_u16;
        
        // let mut addr = 0x9C00_u16; <-- This is where the other tileset starts

        let addressing_mode = self.get_lcd_control_flag(LcdControlFlag::BgAndWindowEnable);
        println!("Addressing Mode: {}", addressing_mode);

        for tile_row in 0..32_usize {
            for tile_col in 0..32_usize {
                let tile_index = self.mmu.borrow().read_byte(addr);
                // println!("{:04x}: {:02x}", addr, tile_index);
                let tile = self.get_tile(tile_index);
                // let tile = self.get_tile_direct_index(0x8000 + tile_index as u16 * 16);
                // let tile = self.get_test_tile(TEST_TILE_RAW);
                addr += 1;
                for pixel_row in 0..8_usize {
                    for pixel_col in 0..8_usize {
                        self.display[pixel_row + tile_row * 8][pixel_col + tile_col * 8] =
                            tile[pixel_row][pixel_col];
                    }
                }
            }
        }
    }

    fn fetch_pixel_slice() {
        unimplemented!()
        // Get tile ID
        // Get tile row (low)
        // Get tile row (high)
        // Push pixels
    }
}

mod debug {
     pub fn get_test_tile(&self, tile_raw: [u8; 16]) -> Tile {
        let mut tile: Tile = [[0; TILE_WIDTH_IN_PIXELS]; TILE_HEIGHT_IN_PIXELS];
        let tile_start_addr = 0;
        for (tile_row_index, tile_row) in tile.iter_mut().enumerate() {
            // Each row contains 2 bytes of information
            let byte1_addr = tile_start_addr + (tile_row_index as u16) * 2;
            let byte2_addr = byte1_addr + 1;
            let byte1 = tile_raw[byte1_addr as usize];
            let byte2 = tile_raw[byte2_addr as usize];

            *tile_row = get_tile_row(byte1, byte2);
        }

        tile
    }

    pub fn get_tile_direct_index(&self, tile_start_addr: u16) -> Tile {
        let mut tile: Tile = [[0; TILE_WIDTH_IN_PIXELS]; TILE_HEIGHT_IN_PIXELS];
        for (tile_row_index, tile_row) in tile.iter_mut().enumerate() {
            // Each row contains 2 bytes of information
            let byte1_addr = tile_start_addr + (tile_row_index as u16) * 2;
            let byte2_addr = byte1_addr + 1;
            let byte1 = self.mmu.borrow().read_byte(byte1_addr);
            let byte2 = self.mmu.borrow().read_byte(byte2_addr);

            *tile_row = get_tile_row(byte1, byte2);
        }

        tile
    } 
}