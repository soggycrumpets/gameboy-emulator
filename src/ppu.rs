mod registers;
mod tile_maps;
mod tiles;

use crate::mmu;
use mmu::Mmu;
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

    pub fn draw(&mut self) {
        // For now, just look through a whole tileset
        let mut addr = 0x9000_u16;
        // let mut addr = 0x9C00_u16;

        for tile_row in 0..32_usize {
            for tile_col in 0..32_usize {
                let tile_index = self.mmu.borrow().read_byte(addr);
                let tile = self.get_tile(tile_index);
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
