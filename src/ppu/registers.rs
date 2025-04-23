use super::Ppu;
use crate::{mmu::memmap::LCDC_ADDR, util::{get_bit, set_bit}};

impl Ppu {
    pub fn get_lcdc_flag(&self, bit: u8) -> bool {
        let byte = self.read_byte(LCDC_ADDR);
        get_bit(byte, bit)
    }

    pub fn set_lcdc_flag(&mut self, bit: u8, set: bool) {
        let mut byte = self.read_byte(LCDC_ADDR);
        set_bit(&mut byte, bit, set);
        self.mmu.borrow_mut().write_byte(LCDC_ADDR, byte);
    }
}