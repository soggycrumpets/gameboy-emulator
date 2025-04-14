pub mod memmap;
use std::{cell::RefCell, rc::Rc};

use memmap::*;

#[derive(Debug)]
pub struct Mmu {
    pub rom_bank_00: [u8; ROM_BANK_0_SIZE],
    pub rom_bank_01: [u8; ROM_BANK_1_SIZE],
    pub vram: [u8; VRAM_SIZE],
    pub exram: [u8; EXRAM_SIZE],
    pub wram_0: [u8; WRAM_0_SIZE],
    pub wram_1: [u8; WRAM_1_SIZE],
    pub echo_ram: [u8; ECHO_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub restricted_memory: [u8; RESTRICTED_MEM_SIZE],
    pub io: [u8; IO_SIZE],
    pub hram: [u8; HRAM_SIZE],
    pub ie: u8,
}

impl Mmu {
    pub fn new() -> Rc<RefCell<Mmu>> {
        let mmu = Mmu {
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

    // Load rom into memory
    pub fn load_rom(&mut self, path: &str) -> bool {
        let rom = match std::fs::read(path) {
            Ok(result) => result,
            Err(..) => return false,
        };

        // TODO: This currently does not support roms with switchable memory banks
        let (rom_half_1, rom_half_2) = rom.split_at(self.rom_bank_00.len());

        self.rom_bank_00.copy_from_slice(rom_half_1);
        self.rom_bank_01.copy_from_slice(rom_half_2);
        
        true
    }

    // ----- Reading and Writing Memory -----

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
            M::Io => self.io[index] = byte,
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
}
