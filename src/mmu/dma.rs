use crate::constants::T_CYCLES_PER_M_CYCLE;

use super::{
    Mmu,
    memmap::{DMA_ADDR, OAM_START, map_addr},
};

pub const DMA_BYTE_TRANSFER_AMOUNT: u16 = 160;
pub const DMA_TRANSFER_T_CYCLES: u16 = DMA_BYTE_TRANSFER_AMOUNT * T_CYCLES_PER_M_CYCLE as u16;
pub const DMA_TARGET_START_ADDR: u16 = 0xFE00;
pub const DMA_TARGET_END_ADDR: u16 = DMA_TARGET_START_ADDR + DMA_BYTE_TRANSFER_AMOUNT;

pub struct Dma {
    timer: u32,
    active: bool,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            timer: 0,
            active: false,
        }
    }
}

impl Mmu {
    pub fn start_dma_transfer(&mut self, dma_byte: u8) {

        // The DMA register needs to be updated first
        let (_, addr_mapped) = map_addr(DMA_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = dma_byte;

        // DMA transfer copies 160 bytes from a source, specified by by the DMA register, to OAM.
        let source_start_addr = (dma_byte as u16) << 8;
        let source_end_addr = source_start_addr + DMA_BYTE_TRANSFER_AMOUNT as u16;

        let mut target_addr = DMA_TARGET_START_ADDR;
        for source_addr in source_start_addr..=source_end_addr {
            let byte = self.read_byte(source_addr);
            self.write_byte(target_addr, byte);

            target_addr += 1;

            println!("{:02x}: {:04x} => {:04x}", byte, source_addr, target_addr);
        }
    }

    pub fn tick_dma(&mut self) {
        if !self.dma.active {
            return;
        }

        self.dma.timer.saturating_sub(1);
        if self.dma.timer == 0 {
            self.dma.active = false;
        }

        if self.dma.timer % T_CYCLES_PER_M_CYCLE != 0 {
            return;
        }

        let dma_byte = self.read_byte(DMA_ADDR);
    }
}
