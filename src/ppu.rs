mod registers;
mod tiles;

use crate::mmu;
use registers::{};
use mmu::Mmu;
use std::{cell::RefCell, rc::Rc};

pub struct Ppu {
    mmu: Rc<RefCell<Mmu>>,
}

impl Ppu {
    pub fn new(mmu: Rc<RefCell<Mmu>>) -> Self {
        Ppu {
            mmu,
        }
    }
}
