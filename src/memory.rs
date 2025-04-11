use crate::constants::PROGRAM_START_ADDR;

pub struct MMU {
    pub memory: [u8; 65536],
}

impl MMU {
    pub fn new() -> MMU {
        MMU { memory: [0; 65536] }
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

    pub fn readbyte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn writebyte(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize] = byte;
    }

    pub fn readword(&self, addr: u16) -> u16 {
        let lowbyte = self.readbyte(addr);
        let highbyte = self.readbyte(addr + 1);
        lowbyte as u16 | ((highbyte as u16) << 8)
    }

    pub fn writeword(&mut self, addr: u16, word: u16) {
        let lowbyte = word as u8;
        let highbyte = (word >> 8) as u8;
        self.writebyte(addr, lowbyte);
        self.writebyte(addr + 1, highbyte);
    }
}
