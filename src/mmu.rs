mod memmap;
use memmap::*;
pub struct Mmu {
    pub memory: [u8; 65536],
    pub rom_bank_00: [u8; ROM_BANK_0_SIZE],
    pub rom_bank_01: [u8; ROM_BANK_1_SIZE],
    pub vram: [u8; VRAM_SIZE],
    pub exram: [u8; EXRAM_SIZE],
    pub wram_0: [u8; WRAM_0_SIZE],
    pub wram_1: [u8; WRAM_1_SIZE],
    pub echo_ram: [u8; ECHO_RAM_SIZE],
    pub oam: [u8; OAM_SIZE],
    pub restricted_memory: [u8; RESTRICTED_MEMORY_SIZE],
    pub io: [u8; IO_SIZE],
    pub hram: [u8; HRAM_SIZE],
    pub ie: u8,
}



impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            memory: [0; 65536],
            rom_bank_00: [0; ROM_BANK_0_SIZE],
            rom_bank_01: [0; ROM_BANK_1_SIZE],
            vram: [0; VRAM_SIZE],
            exram: [0; EXRAM_SIZE],
            wram_0: [0; WRAM_0_SIZE],
            wram_1: [0; WRAM_1_SIZE],
            echo_ram: [0; ECHO_RAM_SIZE],
            oam: [0; OAM_SIZE],
            restricted_memory: [0; RESTRICTED_MEMORY_SIZE],
            io: [0; IO_SIZE],
            hram: [0; HRAM_SIZE],
            ie: 0,
        }
    }

    // Load rom into memory
    pub fn load_rom(&mut self, path: &str, load_start_address: u16) -> bool {
        let rom = match std::fs::read(path) {
            Ok(result) => result,
            Err(..) => return false,
        };
        self.memory[(load_start_address as usize)..(load_start_address as usize + rom.len())]
            .copy_from_slice(&rom);
        true
    }


    // ----- Reading and Writing Memory -----

   


    pub fn readbyte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize] = byte;
    }

    // Pay extra special attentian here to account for little-endianness
    pub fn readword(&self, addr: u16) -> u16 {
        let lowbyte = self.readbyte(addr);
        let highbyte = self.readbyte(addr + 1);
        lowbyte as u16 | ((highbyte as u16) << 8)
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        let lowbyte = word as u8;
        let highbyte = (word >> 8) as u8;
        self.write_byte(addr, lowbyte);
        self.write_byte(addr + 1, highbyte);
    }
}
