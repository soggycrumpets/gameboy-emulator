const TRANSFER_REQUESTED_VALUE: u8 = 0x81;

pub mod memmap;
mod timers;
use memmap::*;
use timers::Timers;
use std::{cell::RefCell, rc::Rc};

use crate::util::set_bit;

#[derive(Debug)]
pub struct Mmu {
    timers: Timers,
    rom_bank_00: [u8; ROM_BANK_0_SIZE],
    rom_bank_01: [u8; ROM_BANK_1_SIZE],
    vram: [u8; VRAM_SIZE],
    exram: [u8; EXRAM_SIZE],
    wram_0: [u8; WRAM_0_SIZE],
    wram_1: [u8; WRAM_1_SIZE],
    echo_ram: [u8; ECHO_RAM_SIZE],
    oam: [u8; OAM_SIZE],
    restricted_memory: [u8; RESTRICTED_MEM_SIZE],
    io: [u8; IO_SIZE],
    hram: [u8; HRAM_SIZE],
    ie: u8,
}

impl Mmu {
    pub fn new() -> Rc<RefCell<Mmu>> {
        let mmu = Mmu {
            timers: Timers::new(),
            rom_bank_00: [0; ROM_BANK_0_SIZE],
            rom_bank_01: [0; ROM_BANK_1_SIZE],
            vram: [0; VRAM_SIZE],
            exram: [0; EXRAM_SIZE],
            wram_0: [0; WRAM_0_SIZE],
            wram_1: [0; WRAM_1_SIZE],
            echo_ram: [0; ECHO_RAM_SIZE],
            oam: [0; OAM_SIZE],
            restricted_memory: [0; RESTRICTED_MEM_SIZE],
            io: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            ie: 0,
        };

        Rc::new(RefCell::new(mmu))
    }

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

    pub fn read_byte(&self, addr: u16) -> u8 {
        let (mem_region, addr_mapped) = map_address(addr);
        let index = addr_mapped as usize;

        use MemRegion as M;
        match mem_region {
            M::RomBank0 => self.rom_bank_00[index],
            M::RomBank1 => self.rom_bank_01[index],
            M::Vram => self.vram[index],
            M::Exram => self.exram[index],
            M::Wram0 => self.wram_0[index],
            M::Wram1 => self.wram_1[index],
            M::EchoRam => self.echo_ram[index],
            M::Oam => self.oam[index],
            M::Restricted => self.restricted_memory[index],
            M::Io => self.io[index],
            M::Hram => self.hram[index],
            M::Ie => self.ie,
        }
    }

    // todo! Some writes and reads work differently for different memory spaces
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let (mem_region, addr_mapped) = map_address(addr);
        let index = addr_mapped as usize;

        if (addr == SERIAL_TRANSFER_CONTROL_ADDR) && (byte == TRANSFER_REQUESTED_VALUE) {
            let c = self.read_byte(SERIAL_TRANSFER_DATA_ADDR) as char;
            print!("{}", c);
        }

        // match addr {
        //     TAC_ADDR => println!("TAC: {}", byte),
        //     TIMA_ADDR => println!("TIMA: {}", byte),
        //     _ => (),
        // }

        use MemRegion as M;
        match mem_region {
            M::RomBank0 => self.rom_bank_00[index] = byte,
            M::RomBank1 => self.rom_bank_01[index] = byte,
            M::Vram => self.vram[index] = byte,
            M::Exram => self.exram[index] = byte,
            M::Wram0 => self.wram_0[index] = byte,
            M::Wram1 => self.wram_1[index] = byte,
            M::EchoRam => self.echo_ram[index] = byte,
            M::Oam => self.oam[index] = byte,
            M::Restricted => self.restricted_memory[index] = byte,
            // IO Has some special cases
            M::Io => match addr {
                // Writes to DIV do not affect the memory directly. They reset the system clock
                DIV_ADDR => self.timers.system_clock = 0,
                _ => self.io[index] = byte,
            },
            M::Hram => self.hram[index] = byte,
            M::Ie => self.ie = byte,
        };
    }

    // Pay extra special attentian here to account for little-endianness
    pub fn read_word(&self, addr: u16) -> u16 {
        let lowbyte = self.read_byte(addr);
        let highbyte = self.read_byte(addr + 1);
        lowbyte as u16 | ((highbyte as u16) << 8)
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        let lowbyte = word as u8;
        let highbyte = (word >> 8) as u8;
        self.write_byte(addr, lowbyte);
        self.write_byte(addr + 1, highbyte);
    }

    // An interrupt is requested by setting a specific bit in the IF register
    pub fn request_interrupt(&mut self, interrupt_bit: u8) {
        let mut byte = self.read_byte(IF_ADDR);
        set_bit(&mut byte, interrupt_bit, true);
        self.write_byte(IF_ADDR, byte);
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
    fn test_readwrite_memory_mapping() {
        // Declare two distinct bytes
        let first_byte: u8 = 0b_0110;
        let last_byte: u8 = 0b_1001;

        let mmu = Mmu::new();

        let start_addresses = [
            ROM_BANK_0_START,
            ROM_BANK_1_START,
            VRAM_START,
            EXRAM_START,
            WRAM_0_START,
            WRAM_1_START,
            ECHO_RAM_START,
            OAM_START,
            RESTRICTED_MEM_START,
            IO_START,
            HRAM_START,
            IE_ADDR,
        ];

        let end_addresses = [
            ROM_BANK_0_END,
            ROM_BANK_1_END,
            VRAM_END,
            EXRAM_END,
            WRAM_0_END,
            WRAM_1_END,
            ECHO_RAM_END,
            OAM_END,
            RESTRICTED_MEM_END,
            IO_END,
            HRAM_END,
            IE_ADDR,
        ];

        // Make sure that each memory region has its first and last bytes free
        assert_eq!(mmu.borrow().rom_bank_00[0], 0);
        assert_eq!(mmu.borrow().rom_bank_01[0], 0);
        assert_eq!(mmu.borrow().vram[0], 0);
        assert_eq!(mmu.borrow().exram[0], 0);
        assert_eq!(mmu.borrow().wram_0[0], 0);
        assert_eq!(mmu.borrow().echo_ram[0], 0);
        assert_eq!(mmu.borrow().oam[0], 0);
        assert_eq!(mmu.borrow().restricted_memory[0], 0);
        assert_eq!(mmu.borrow().io[0], 0);
        assert_eq!(mmu.borrow().hram[0], 0);

        assert_eq!(*mmu.borrow().rom_bank_00.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().rom_bank_01.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().vram.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().exram.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().wram_0.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().echo_ram.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().oam.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().restricted_memory.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().io.last().unwrap(), 0);
        assert_eq!(*mmu.borrow().hram.last().unwrap(), 0);

        assert_eq!(mmu.borrow().ie, 0);

        // Get the bytes into memory
        for i in 0..start_addresses.len() {
            let start_address = start_addresses[i];
            let end_address = end_addresses[i];
            mmu.borrow_mut().write_byte(start_address, first_byte);
            mmu.borrow_mut().write_byte(end_address, last_byte);
        }

        // See if the write worked
        assert_eq!(mmu.borrow().rom_bank_00[0], first_byte);
        assert_eq!(mmu.borrow().rom_bank_01[0], first_byte);
        assert_eq!(mmu.borrow().vram[0], first_byte);
        assert_eq!(mmu.borrow().exram[0], first_byte);
        assert_eq!(mmu.borrow().wram_0[0], first_byte);
        assert_eq!(mmu.borrow().echo_ram[0], first_byte);
        assert_eq!(mmu.borrow().oam[0], first_byte);
        assert_eq!(mmu.borrow().restricted_memory[0], first_byte);
        assert_eq!(mmu.borrow().io[0], first_byte);
        assert_eq!(mmu.borrow().hram[0], first_byte);

        assert_eq!(*mmu.borrow().rom_bank_00.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().rom_bank_01.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().vram.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().exram.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().wram_0.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().echo_ram.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().oam.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().restricted_memory.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().io.last().unwrap(), last_byte);
        assert_eq!(*mmu.borrow().hram.last().unwrap(), last_byte);

        // Make sure the read method matches the direct indexing
        for i in 0..start_addresses.len() {
            let start_address = start_addresses[i];
            let end_address = end_addresses[i];

            let first_byte_reading = mmu.borrow().read_byte(start_address);
            let last_byte_reading = mmu.borrow().read_byte(end_address);

            // The IE register has been overwritten, so it should NOT be equal.
            if start_address != end_address {
                assert_eq!(first_byte_reading, first_byte);
            } else {
                assert_ne!(first_byte_reading, first_byte);
            }
            assert_eq!(last_byte_reading, last_byte)
        }
    }
}
