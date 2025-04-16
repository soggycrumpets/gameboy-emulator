use sdl2::libc::MODULE_INIT_IGNORE_MODVERSIONS;

use super::Ppu;
use crate::util::{set_bit, get_bit};

// Ppu mode bit positions
const HBLANK_MODE_NUMBER: u8 = 0;
const VBLANK_MODE_NUMBER: u8 = 1;
const OAM_SCAN_MODE_NUMBER: u8 = 2;
const PIXEL_DRAW_MODE_NUMBER: u8 = 3;

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Register {
    Lcdc = 0xFF40, // LCD control
    Stat = 0xFF41, // LCD status
    Ly = 0xFF44, // Line y Position
    Lyc = 0xFF45, // Line y position compare
    Scy = 0xFF42, // Viewport y position
    Scx = 0xFF43, // Viewport x position
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum LcdControlFlag {
    // Bits in the LCDC register
    LcdAndPpuEnabled = 7,
    WindowTileMap = 6,
    WindowEnable = 5,
    BgAndWindowTiles = 4,
    BgTileMap = 3,
    ObjSize = 2,
    ObjEnable = 1,
    BgAndWindowEnable = 0,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum LcdStatusFlag {
    // Bits in the STAT register
    LycIntSelect = 6,
    Mode2IntSelect = 5,
    Mode1IntSelect = 4,
    Mode0IntSelect = 3,
    LycEqualsLy = 2,
}

// This is technically bits 0 and 1 of LcdStatus, but I separated 
// it out because it has its own "flags".
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum PpuMode {
    HBlank = HBLANK_MODE_NUMBER,
    VBlank = VBLANK_MODE_NUMBER,
    OamScan = OAM_SCAN_MODE_NUMBER,
    PixelDraw = PIXEL_DRAW_MODE_NUMBER,
}

impl Ppu {
    pub fn get_register(&self, register: Register) -> u8 {
        self.mmu.borrow().read_byte(register as u16)
    }

    pub fn set_register(&self, register: Register, byte: u8) {
        self.mmu.borrow_mut().write_byte(register as u16, byte);
    }

    pub fn get_lcd_control_flag(&self, flag: LcdControlFlag) -> bool {
        let byte = self.get_register(Register::Lcdc);
        get_bit(byte, flag as u8)
    }

    pub fn set_lcd_control_flag(&mut self, flag: LcdControlFlag, set: bool) {
        let mut byte = self.get_register(Register::Lcdc);
        byte = set_bit(byte, flag as u8, set);
        self.set_register(Register::Lcdc, byte);
    }

    pub fn get_lcd_status_flag(&self, flag: LcdStatusFlag) -> bool {
        let byte = self.get_register(Register::Stat);
        get_bit(byte, flag as u8)
    }

    pub fn set_lcd_status_flag(&mut self, flag: LcdStatusFlag, set: bool) {
        let mut byte = self.get_register(Register::Stat);
        byte = set_bit(byte, flag as u8, set);
        self.set_register(Register::Stat, byte);
    }

    pub fn get_mode(&mut self) -> PpuMode {
        // The mode is represented by the rightmost two bits of the LCDC register.
        let byte = self.get_register(Register::Lcdc);
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
        let mut byte = self.get_register(Register::Lcdc);
        byte &= 0b_1111_1100;
        byte |= mode_number;
        self.set_register(Register::Lcdc, byte);
    }
}

#[cfg(test)]
mod tests {
    use crate::create_gameboy_components;
    use super::*;
    #[test]
    fn test_get_and_set_mode() {
        let (_mmu, _cpu, mut ppu) = create_gameboy_components();

        let mut byte = ppu.get_register(Register::Lcdc);
        assert_eq!(byte, 0);

        let set_byte: u8 = 0b_1111_1111;
        let expected_byte: u8 = 0b_1111_1100;
        ppu.set_register(Register::Lcdc, set_byte);

        let mode = ppu.get_mode();
        assert_eq!(mode, PpuMode::PixelDraw);

        byte = ppu.get_register(Register::Lcdc);
        assert_eq!(byte, set_byte);

        ppu.set_mode(PpuMode::HBlank);
        byte = ppu.get_register(Register::Lcdc);
        assert_eq!(byte, expected_byte)
    }    
}
