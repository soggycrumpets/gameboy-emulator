mod registers;

use crate::mmu;
use registers::{LcdControl, LcdStatus};
use mmu::Mmu;
use std::{cell::RefCell, rc::Rc};

enum PpuMode {
    HBlank,
    VBlank,
    OamScan,
    Draw,
}

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
    mode: PpuMode,

    // Registers
    lcdc: LcdControl, // LCD Control
    ly: u8, // LCD Y Coordinate
    lyc: u8, // LCD Y Compare
    stat: LcdStatus, // LCD Status
    scy: u8, // Background viewport y
    scx: u8, // Background viewport x
}

impl Ppu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Self {
        Ppu {
            mmu,
            mode: PpuMode::HBlank,

            lcdc: LcdControl::new(),
            ly: 0,
            lyc: 0,
            stat: LcdStatus::new(),
            scy: 0,
            scx: 0,
        }
    }
}
