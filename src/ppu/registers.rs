use super::Ppu;
use crate::{mmu::memmap::{LCDC_ADDR, STAT_ADDR}, util::{get_bit, set_bit}};

// Ppu mode bit positions
// This is technically bits 0 and 1 of LcdStatus, but I separated 
// it out because it has its own "flags".


impl Ppu {

    pub fn get_lcdc_flag(&self, bit: u8) -> bool {
        let byte = self.read_byte(LCDC_ADDR);
        get_bit(byte, bit)
    }
}

#[cfg(test)]
mod tests {
    use crate::create_gameboy_components;
    use super::*;
    // #[test]
    // fn test_get_and_set_mode() {
    //     let (_mmu, _cpu, mut ppu) = create_gameboy_components();

    //     let mut byte = ppu.read_byte(LCDC_ADDR);
    //     assert_eq!(byte, 0);

    //     let set_byte: u8 = 0b_1111_1111;
    //     let expected_byte: u8 = 0b_1111_1100;
    //     ppu.set_register(Register::Lcdc, set_byte);

    //     let mode = ppu.get_mode();
    //     assert_eq!(mode, PpuMode::PixelDraw);

    //     byte = ppu.get_register(Register::Lcdc);
    //     assert_eq!(byte, set_byte);

    //     ppu.set_mode(PpuMode::HBlank);
    //     byte = ppu.get_register(Register::Lcdc);
    //     assert_eq!(byte, expected_byte)
    // }    
}
