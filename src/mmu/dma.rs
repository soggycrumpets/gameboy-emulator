use crate::constants::T_CYCLES_PER_M_CYCLE;

use super::{
    Mmu,
    memmap::{DMA_ADDR, map_addr},
};

const DMA_BYTE_TRANSFER_AMOUNT: u16 = 160;
const DMA_TRANSFER_T_DELAY: u16 = 4;
const DMA_TRANSFER_T_CYCLES: u16 =
    DMA_BYTE_TRANSFER_AMOUNT * T_CYCLES_PER_M_CYCLE as u16 + DMA_TRANSFER_T_DELAY;
const DMA_TARGET_START_ADDR: u16 = 0xFE00;

// todo! This is bad!
const FUDGE_FACTOR: u16 = 8;

pub struct Dma {
    timer: u16,
    pub active: bool,
    pending: bool,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            timer: 0,
            active: false,
            pending: false,
        }
    }
}

impl Mmu {
    pub fn start_dma_transfer(&mut self, dma_byte: u8) {
        self.dma.active = true;
        self.dma.timer = DMA_TRANSFER_T_CYCLES + FUDGE_FACTOR;

        // The DMA register needs to be updated first
        let (_, addr_mapped) = map_addr(DMA_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = dma_byte;

        // DMA transfer copies 160 bytes from a source, specified by by the DMA register, to OAM.
        let source_start_addr = (dma_byte as u16) << 8;

        let mut source_addr = source_start_addr;
        let mut target_addr = DMA_TARGET_START_ADDR;
        for _i in 0..DMA_BYTE_TRANSFER_AMOUNT {
            let byte = self.read_byte(source_addr);
            self.write_byte_override(target_addr, byte); // Bypass the DMA lock

            source_addr += 1;
            target_addr += 1;

            // println!("{:02x}: {:04x} => {:04x}", byte, source_addr, target_addr);
        }
    }

    pub fn tick_dma(&mut self) {

        if !self.dma.active {
            return;
        }

        self.dma.timer = self.dma.timer.saturating_sub(1);
        if self.dma.timer == 0 {
            self.dma.active = false;
        }

        if self.dma.timer % (T_CYCLES_PER_M_CYCLE as u16) != 0 {
            return;
        }


        // Copy data one byte at a time
    }
}
