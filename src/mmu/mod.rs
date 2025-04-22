//! The MMU stores, controls, and regulates memory.
//! The Gameboy has 64kB of memory, but it cannot be sufficiently represented
//! by a single 64kB array. This is primarily due to the fact that the Gameboy supports
//! ROM bank switching, which changes the part of the game's ROM that is visible in memory.
//! 
//! Different regions of memory are treated in different ways by different pieces of hardware on
//! the Gameboy. For example, the CPU is restricted from accessing VRAM and OAM during certain
//! timing windows. Certain registers, such as DIV and TIMA, incur side effects when written to.
 
pub mod memmap;
mod readwrite;
mod dma;
mod timers;

use dma::Dma;
use memmap::*;
use std::{cell::RefCell, rc::Rc};
use timers::Timers;

use crate::util::set_bit;

pub struct Mmu {
    dma: Dma,
    timers: Timers,
    rom_bank_00: [u8; ROM_BANK_0_SIZE],
    rom_bank_01: [u8; ROM_BANK_1_SIZE],
    vram: [u8; VRAM_SIZE],
    exram: [u8; EXRAM_SIZE],
    wram_0: [u8; WRAM_0_SIZE],
    wram_1: [u8; WRAM_1_SIZE],
    oam: [u8; OAM_SIZE],
    restricted_memory: [u8; RESTRICTED_MEM_SIZE],
    io: [u8; IO_SIZE],
    hram: [u8; HRAM_SIZE],
    ie: u8,

    pub vram_lock: bool,
    pub oam_lock: bool,
}

impl Mmu {
    //! I chose to use the Rc RefCell for the MMU so that the CPU and PPU could borrow a mutable
    //! reference to access it whenever they need to. It could just as easily be a global variable.
    pub fn new() -> Rc<RefCell<Mmu>> {
        let mmu = Mmu {
            dma: Dma::new(),
            timers: Timers::new(),
            rom_bank_00: [0; ROM_BANK_0_SIZE],
            rom_bank_01: [0; ROM_BANK_1_SIZE],
            vram: [0; VRAM_SIZE],
            exram: [0; EXRAM_SIZE],
            wram_0: [0; WRAM_0_SIZE],
            wram_1: [0; WRAM_1_SIZE],
            oam: [0; OAM_SIZE],
            restricted_memory: [0; RESTRICTED_MEM_SIZE],
            io: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            ie: 0,

            vram_lock: false,
            oam_lock: false,
        };

        Rc::new(RefCell::new(mmu))
    }

    // todo! Support bigger ROMs
    pub fn load_rom(&mut self, path: &str) -> bool {
        let rom = match std::fs::read(path) {
            Ok(result) => result,
            Err(..) => return false,
        };
        for (addr, byte) in rom.iter().enumerate() {
            self.write_byte(addr as u16, *byte);
        }

        true
    }

   
}

mod debug {
    use super::*;

    impl Mmu {
        pub fn print_vram(&self) {
            println!("\n\n\n\n\n\nVRAM:");
            for (byte_number, byte) in self.vram.iter().enumerate() {
                print!("{:02x} ", byte);
                if byte_number % 32 == 31 {
                    println!();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::create_gameboy_components;

    use super::*;

    // For now, this is just a non-zero number that I picked.
    const BYTE: u8 = 0b_0110;
    // The first and second byte should be different, to make sure that they are in the right order
    const WORD: u16 = 0b_1001_0110;
    const ADDR: u16 = PROGRAM_START_ADDR;

    #[test]
    fn test_read_write_byte() {
        let mmu = Mmu::new();
        let mut byte_reading;

        // Make sure the memory starts uninitialized
        byte_reading = mmu.borrow().read_byte(ADDR);
        assert_eq!(byte_reading, 0);

        // Write to memory
        mmu.borrow_mut().write_byte(ADDR, BYTE);

        // Make sure the memory has changed to what we expect
        byte_reading = mmu.borrow().read_byte(ADDR);
        assert_eq!(byte_reading, BYTE);
    }

    #[test]
    fn test_read_write_word() {
        let mmu = Mmu::new();
        let mut word_reading;

        // Make sure the memory starts uninitialized
        word_reading = mmu.borrow().read_word(ADDR);
        assert_eq!(word_reading, 0);

        // Write to memory
        mmu.borrow_mut().write_word(ADDR, WORD);

        // Make sure the memory has changed to what we expect
        word_reading = mmu.borrow().read_word(ADDR);
        assert_eq!(word_reading, WORD);
    }

    #[test]
    fn test_echo_ram() {
        let (mmu, _, _) = create_gameboy_components();
        let target_byte = 0xFF;

        // Writes to wram mirror to echo ram
        {
            let initial_byte = mmu.borrow().read_byte(ECHO_RAM_START);
            assert_ne!(target_byte, initial_byte);

            mmu.borrow_mut().write_byte(WRAM_0_START, target_byte);

            let echoed_byte = mmu.borrow().read_byte(ECHO_RAM_START);
            assert_eq!(target_byte, echoed_byte);
        }

        {
            // Writes to echo ram occur in wram (and work with wram1)
            let initial_byte = mmu.borrow().read_byte(WRAM_1_START);
            assert_ne!(target_byte, initial_byte);

            mmu.borrow_mut()
                .write_byte(WRAM_1_START + ECHO_OFFSET, target_byte);

            let echoed_byte = mmu.borrow().read_byte(WRAM_1_START);
            assert_eq!(target_byte, echoed_byte);
        }
    }
}
