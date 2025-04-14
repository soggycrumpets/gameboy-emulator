use super::Ppu;
use crate::util::{set_bit, get_bit};

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Register {
    Lcdc = 0xFF40,
    Stat = 0xFF41,
    Ly = 0xFF44,
    Lyc = 0xFF45,
    Scy = 0xFF42,
    Scx = 0xFF43,
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
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    PixelDraw = 3,
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
}
