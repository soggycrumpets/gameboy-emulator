const TRANSFER_REQUESTED_VALUE: u8 = 0x81;
const GARBAGE_VALUE: u8 = 0xFF;

mod dma;
pub mod memmap;
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

    // Memory regions are all treated separately, and lots of regions and addresses
    // have special rules that determine what happens when a read or write is done.
    pub fn read_byte(&self, addr: u16) -> u8 {
        let (mem_region, addr_mapped) = map_addr(addr);
        let index = addr_mapped as usize;

        use MemRegion as M;
        match mem_region {
            M::RomBank0 => self.rom_bank_00[index],
            M::RomBank1 => self.rom_bank_01[index],
            M::Vram => {
                if self.vram_lock {
                    GARBAGE_VALUE
                } else {
                    self.vram[index]
                }
            }
            M::Exram => self.exram[index],
            M::Wram0 => self.wram_0[index],
            M::Wram1 => self.wram_1[index],
            M::EchoRam => self.read_byte(addr - ECHO_OFFSET),
            M::Oam => {
                if self.oam_lock || self.dma.active {
                    GARBAGE_VALUE
                } else {
                    self.oam[index]
                }
            }
            M::Restricted => self.restricted_memory[index],
            M::Io => match addr {
                P1_ADDR => 0x0f,
                IF_ADDR=> self.io[index] | 0b_1110_0000, // Upper 3 bits always read high
                _ => self.io[index],
            }
            M::Hram => self.hram[index],
            M::Ie => self.ie,
        }
    }

    // Ignore all special rules
    pub fn read_byte_override(&mut self, addr: u16) -> u8 {
        let (mem_region, addr_mapped) = map_addr(addr);
        let index = addr_mapped as usize;

        use MemRegion as M;
        match mem_region {
            M::RomBank0 => self.rom_bank_00[index],
            M::RomBank1 => self.rom_bank_01[index],
            M::Vram => self.vram[index],
            M::Exram => self.exram[index],
            M::Wram0 => self.wram_0[index],
            M::Wram1 => self.wram_1[index],
            M::EchoRam => self.read_byte(addr - ECHO_OFFSET),
            M::Oam => self.oam[index],
            M::Restricted => self.restricted_memory[index],
            M::Io => self.io[index],
            M::Hram => self.hram[index],
            M::Ie => self.ie,
        }
    }

    // The PPU is not affected by the vram lock
    pub fn bypass_read_byte_vram(&self, addr: u16) -> u8 {
        let (mem_region, addr_mapped) = map_addr(addr);
        if mem_region != MemRegion::Vram {
            self.read_byte(addr)
        } else {
            let index = addr_mapped as usize;
            self.vram[index]
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let (mem_region, addr_mapped) = map_addr(addr);
        let index = addr_mapped as usize;

        if (addr == SC_ADDR) && (byte == TRANSFER_REQUESTED_VALUE) {
            let c = self.read_byte(SB_ADDR) as char;
            print!("{}", c);
        }

        use MemRegion as M;
        match mem_region {
            M::RomBank0 => self.rom_bank_00[index] = byte,
            M::RomBank1 => self.rom_bank_01[index] = byte,
            M::Vram => {
                if !self.vram_lock {
                    self.vram[index] = byte
                }
            }
            M::Exram => self.exram[index] = byte,
            M::Wram0 => self.wram_0[index] = byte,
            M::Wram1 => self.wram_1[index] = byte,
            M::EchoRam => self.write_byte(addr - ECHO_OFFSET, byte),
            M::Oam => {
                if !self.oam_lock && !self.dma.active {
                    self.oam[index] = byte
                }
            }
            M::Restricted => self.restricted_memory[index] = byte,
            // IO writes have special behaviors
            M::Io => match addr {
                DIV_ADDR => self.write_byte_div(),
                TMA_ADDR => self.write_byte_tma(byte),
                TAC_ADDR => self.write_byte_tac(byte),
                TIMA_ADDR => self.write_byte_tima(byte),
                DMA_ADDR => self.start_dma_transfer(byte),
                LY_ADDR => (),                                     // Read-only
                STAT_ADDR => self.io[index] = byte & 0b_1111_1000, // Bottom 3 bits are read-only
                IF_ADDR => self.io[index] = byte | 0b_1110_0000,   // Top 3 bits are always 1
                _ => self.io[index] = byte,
            },
            M::Hram => self.hram[index] = byte,
            M::Ie => self.ie = byte,
        };
    }

    // During DMA, the CPU only has access to HRAM
    fn dma_write() {}

    // Ignore all special rules (except echo ram)
    pub fn write_byte_override(&mut self, addr: u16, byte: u8) {
        let (mem_region, addr_mapped) = map_addr(addr);
        let index = addr_mapped as usize;

        use MemRegion as M;
        match mem_region {
            M::RomBank0 => self.rom_bank_00[index] = byte,
            M::RomBank1 => self.rom_bank_01[index] = byte,
            M::Vram => self.vram[index] = byte,
            M::Exram => self.exram[index] = byte,
            M::Wram0 => self.wram_0[index] = byte,
            M::Wram1 => self.wram_1[index] = byte,
            M::EchoRam => self.write_byte(addr - ECHO_OFFSET, byte),
            M::Oam => self.oam[index] = byte,
            M::Restricted => self.restricted_memory[index] = byte,
            M::Io => self.io[index] = byte,
            M::Hram => self.hram[index] = byte,
            M::Ie => self.ie = byte,
        };
    }

    // LY is read-only by the CPU, but the PPU needs to write to them.
    pub fn bypass_write_byte_ly(&mut self, byte: u8) {
        let (_mem_region, addr_mapped) = map_addr(LY_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = byte;
    }

    // The same is true for STAT, but only the bottom three bits.
    pub fn bypass_write_byte_stat(&mut self, byte: u8) {
        let (_mem_region, addr_mapped) = map_addr(STAT_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = byte;
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
