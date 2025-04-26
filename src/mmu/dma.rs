use crate::constants::M_CYCLE_DURATION;

use super::{
    Mmu,
    memmap::{DMA_ADDR, map_addr},
};

// todo! This should really be 4. It seems like my timing logic starts DMA a cycle early by default.
// Although, the timing is lining up exactly as it should right now.
// This may ba a "if it ain't broke, don't fix it" type of situation.
const DMA_TRANSFER_T_DELAY: u16 = 8;

const DMA_BYTE_TRANSFER_AMOUNT: u16 = 160;
const DMA_TRANSFER_T_CYCLES: u16 = DMA_BYTE_TRANSFER_AMOUNT * M_CYCLE_DURATION as u16;
const DMA_TARGET_START_ADDR: u16 = 0xFE00;

pub struct Dma {
    timer: u16,
    pub active: bool,
    source_start_addr: u16,
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            timer: 0,
            active: false,
            source_start_addr: 0x0000,
        }
    }
}

impl Mmu {
    pub fn start_dma_transfer(&mut self, dma_byte: u8) {
        // The DMA register needs to be updated first

        let index = map_addr(DMA_ADDR);
        self.io[index] = dma_byte;

        self.dma.timer = DMA_TRANSFER_T_CYCLES + DMA_TRANSFER_T_DELAY;
        self.dma.source_start_addr = (dma_byte as u16) << 8;
    }

    pub fn tick_dma(&mut self) {
        if self.dma.timer == 0 {
            return;
        }

        self.dma.timer = self.dma.timer.saturating_sub(1);

        // This is to account for the delay in starting DMA transfer
        // And to limit its byte transfer rate to one per m-cycle
        if self.dma.timer > DMA_TRANSFER_T_CYCLES || self.dma.timer % (M_CYCLE_DURATION as u16) != 0
        {
            return;
        }
        // println!("CYCLE: {}", self.dma.timer);
        self.oam_lock = true;

        // Copy data one byte at a time
        let current_index = DMA_BYTE_TRANSFER_AMOUNT - (self.dma.timer / M_CYCLE_DURATION as u16);

        let source_addr = self.dma.source_start_addr + current_index;
        let target_addr = DMA_TARGET_START_ADDR + current_index;
        let byte = self.read_byte_override(source_addr);
        self.write_byte_override(target_addr, byte);

        // println!("{:02x}: {:04x} => {:04x}", byte, source_addr, target_addr);

        if self.dma.timer == 0 {
            self.oam_lock = false;
        }
    }
}
