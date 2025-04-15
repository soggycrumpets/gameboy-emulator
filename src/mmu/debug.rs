use super::*;

impl Mmu {
    // Dump the entire rom into memory
    #[cfg(debug_assertions)]
    pub fn load_test_rom(&mut self, path: &str) -> bool {
        
    let rom = match std::fs::read(path) {
        Ok(result) => result,
        Err(..) => return false,
    };

    for (addr, byte) in rom.into_iter().enumerate() {
        self.write_byte(addr as u16, byte);
    }

    true
}
}