use crate::{mmu::memmap::{BG_AND_WINDOW_TILES_BIT, BGP_ADDR}, util::get_bit};

use super::Ppu;
use crate::mmu::Mmu;

// Tiles are stored in VRAM at 0x0000 - 0x94FF.

// A tile is represented by an 8x8 grid of 2-bit integers, laid out
// as 16 consecutive bytes in memory in a very particular format.
type TileRow = [u8; TILE_WIDTH_IN_PIXELS];

pub const TILE_WIDTH_IN_PIXELS: usize = 8;
pub const TILE_HEIGHT_IN_PIXELS: usize = TILE_WIDTH_IN_PIXELS;
const TILE_SIZE_IN_BYTES: usize = 16;

const SIGNED_ADDRESSING_BASE_POINTER: u16 = 0x9000;
const UNSIGNED_ADDRESSING_BASE_POINTER: u16 = 0x8000;

// The Each pixel's color is encoded as a 2-bit number
fn get_pixel_bits(byte1: u8, byte2: u8, col: usize) -> u8 {
    // Col 0 is the leftmost bit
    // Col 7 is the rightmost bit
    let bit_index = (7 - col) as u8;

    let bit1 = get_bit(byte1, bit_index) as u8;
    let bit2 = get_bit(byte2, bit_index) as u8;
    // Bit 2 is the rightmost bit; bit 1 is the leftmost.
    bit1 | (bit2 << 1)
}

// Color palettes are determined by the BGP register for background/window
// Color palettes are determined by OBP1 and OBP2 for objects
// Objects ignore the bottom nibble because their color 0 is transparent.
pub fn apply_palette_to_pixel(pixel_bits: &u8, palette: u8, is_obj: bool) -> u8 {
    match pixel_bits {
        0 => (palette & 0b11) * is_obj as u8,
        1 => (palette >> 2) & 0b11,
        2 => (palette >> 4) & 0b11,
        3 => (palette >> 6) & 0b11,
        _ => panic!("Invalid value for pixel bits: {}", pixel_bits),
    }
}

pub fn get_tile_row(byte1: u8, byte2: u8, mmu: &mut Mmu) -> TileRow {
    let mut row_pixels: TileRow = [0; TILE_WIDTH_IN_PIXELS];
    let bgp = mmu.read_byte(BGP_ADDR);
    for (pixel_index, pixel) in row_pixels.iter_mut().enumerate() {
        let pixel_bits = get_pixel_bits(byte1, byte2, pixel_index);
        *pixel = apply_palette_to_pixel(&pixel_bits, bgp, false)
    }
    row_pixels
}

impl Ppu {
    // The way that they are indexed depends on a register flag.
    pub fn get_tile_start_addr(&self, index: u8, is_object: bool, mmu: &mut Mmu) -> u16 {
        let signed_addressing_mode =
            !is_object && !self.get_lcdc_flag(BG_AND_WINDOW_TILES_BIT, mmu);

        let base_pointer: u16 = if signed_addressing_mode {
            SIGNED_ADDRESSING_BASE_POINTER
        } else {
            UNSIGNED_ADDRESSING_BASE_POINTER
        };

        if signed_addressing_mode {
            let index_signed = index as i8;
            // Unsigned ints are upcasted before doing signed operations, to prevent data loss
            let address_offset = (index_signed as i16).wrapping_mul(TILE_SIZE_IN_BYTES as i16);
            let address = (base_pointer as i32).wrapping_add(address_offset as i32);
            address as u16
        } else {
            let address_offset = (index as u16).wrapping_mul(TILE_SIZE_IN_BYTES as u16);
            base_pointer.wrapping_add(address_offset)
        }
    }

    pub fn get_tile_row_high_byte(
        &mut self,
        tile_start_addr: u16,
        row_index: u8,
        mmu: &mut Mmu,
    ) -> u8 {
        self.read_byte(tile_start_addr + (row_index as u16 * 2) + 1, mmu)
    }

    pub fn get_tile_row_low_byte(
        &mut self,
        tile_start_addr: u16,
        row_index: u8,
        mmu: &mut Mmu,
    ) -> u8 {
        self.read_byte(tile_start_addr + (row_index as u16 * 2), mmu)
    }
}
