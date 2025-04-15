use crate::util::get_bit;

use super::{Ppu, registers::LcdControlFlag};

// Tiles are stored in VRAM at 0x0000 - 0x94FF.

// A tile is represented by an 8x8 grid of 2-bit integers, laid out
// as 16 consecutive bytes in memory in a very particular format.
pub type Tile = [[u8; TILE_WIDTH_IN_PIXELS]; TILE_HEIGHT_IN_PIXELS];
type TileRow = [u8; TILE_WIDTH_IN_PIXELS];

const TILE_WIDTH_IN_PIXELS: usize = 8;
const TILE_HEIGHT_IN_PIXELS: usize = TILE_WIDTH_IN_PIXELS;
const TILE_SIZE_IN_BYTES: usize = 16;

const SIGNED_ADDRESSING_MODE_BASE_POINTER: u16 = 0x9000;
const UNSIGNED_ADDRESSING_MODE_BASE_POINTER: u16 = 0x8000;

// The Each pixel's color is encoded as a 2-bit number
fn get_pixel(byte1: u8, byte2: u8, col: usize) -> u8 {
    // Col 0 is the leftmost bit
    // Col 7 is the rightmost bit
    let bit_index = (7 - col) as u8;

    let bit1 = get_bit(byte1, bit_index) as u8;
    let bit2 = get_bit(byte2, bit_index) as u8;
    // Bit 2 is the rightmost bit; bit 1 is the leftmost.
    bit1 | (bit2 << 1)
}

fn get_tile_row(byte1: u8, byte2: u8) -> TileRow {
    let mut row_pixels: TileRow = [0; TILE_WIDTH_IN_PIXELS];
    for (pixel_index, pixel) in row_pixels.iter_mut().enumerate() {
        *pixel = get_pixel(byte1, byte2, pixel_index);
    }
    row_pixels
}

impl Ppu {
    // The way that they are indexed depends on a register flag.
    fn get_tile_start_addr(&self, index: u8) -> u16 {
        let signed_addressing_mode = self.get_lcd_control_flag(LcdControlFlag::BgAndWindowEnable);
        // The base pointer is different between the two addressing modes
        let bp: u16 = if signed_addressing_mode {
            SIGNED_ADDRESSING_MODE_BASE_POINTER
        } else {
            UNSIGNED_ADDRESSING_MODE_BASE_POINTER
        };

        if signed_addressing_mode {
            let index_signed = index as i8;
            // Unsigned ints are upcasted before doing signed operations, to prevent data loss
            let address_offset = (index_signed as i16).wrapping_mul(TILE_SIZE_IN_BYTES as i16);
            let address = (bp as i32).wrapping_add(address_offset as i32);
            address as u16
        } else {
            let address_offset = (index as u16).wrapping_mul(TILE_SIZE_IN_BYTES as u16);
            bp.wrapping_add(address_offset)
        }
    }

    pub fn get_tile(&self, index: u8) -> Tile {
        let tile_start_addr = self.get_tile_start_addr(index);

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