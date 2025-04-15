use super::{Ppu, registers};
use registers::{LcdControlFlag, Register};
// Tilemap 1 is located at 0x9C00-0x9FFF
// Tilemaps are 32x32 tiles
// Each map contains the 1-byte indices of the tiles to be displayed

const TILE_INDEX_BITS_15_TO_11: u16 = 0b_10011;

impl Ppu {
    fn get_address_bg_mode(&self) -> u16 {
        let bit_10 = self.get_lcd_control_flag(LcdControlFlag::BgTileMap) as u16;

        let ly = self.get_register(Register::Ly) as u16;
        let scy = self.get_register(Register::Scy) as u16;
        let bits_9_to_5 = (ly + scy) / 8;

        // https://github.com/ISSOtm/pandocs/blob/rendering-internals/src/Rendering_Internals.md#STAT%20modes
        todo!("Determine LX (Not a real register)");
        // let lx = self.get_register(Register::Lx) as u16;
        let scx = self.get_register(Register::Scx) as u16;
        // let bits_9_to_5 = (lx + scx) / 8;

        let mut result: u16 = 0;
        result |= TILE_INDEX_BITS_15_TO_11 << 11;
        result |= bit_10 << 10;
        result |= bits_9_to_5 << 5;

    }

    fn get_address_window_mode(&self) {
        let bit_10 = self.get_lcd_control_flag(LcdControlFlag::WindowTileMap) as u16;
    }

    pub fn get_tile_id(&self) {
        todo!("Determine if the PPU is in BG mode vs Window mode");
    }
}
