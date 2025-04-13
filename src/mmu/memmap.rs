use super::Mmu;

pub const ROM_BANK_0_START: u16 = 0x0000;
pub const ROM_BANK_0_END: u16 = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = (ROM_BANK_0_END - ROM_BANK_0_START + 1) as usize;

// Switchable bank
pub const ROM_BANK_1_START: u16 = 0x4000;
pub const ROM_BANK_1_END: u16 = 0x7FFF;
pub const ROM_BANK_1_SIZE: usize = (ROM_BANK_1_END - ROM_BANK_1_START + 1) as usize;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;

// Cartridge's external RAM
pub const EXRAM_START: u16 = 0xA000;
pub const EXRAM_END: u16 = 0xBFFF;
pub const EXRAM_SIZE: usize = (EXRAM_END - EXRAM_START + 1) as usize;

pub const WRAM_0_START: u16 = 0xC000;
pub const WRAM_0_END: u16 = 0xCFFF;
pub const WRAM_0_SIZE: usize = (WRAM_0_END - WRAM_0_START + 1) as usize;

// Sitchable bank 1-7 in CGB mode
pub const WRAM_1_START: u16 = 0xD000;
pub const WRAM_1_END: u16 = 0xDFFF;
pub const WRAM_1_SIZE: usize = (WRAM_1_END - WRAM_1_START + 1) as usize;

pub const ECHO_RAM_START: u16 = 0xE000;
pub const ECHO_RAM_END: u16 = 0xFDFF;
pub const ECHO_RAM_SIZE: usize = (ECHO_RAM_END - ECHO_RAM_START + 1) as usize;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: usize = (OAM_END - OAM_START + 1) as usize;

// Prohibited area of memory
pub const RESTRICTED_MEMORY_START: u16 = 0xFEA0;
pub const RESTRICTED_MEMORY_END: u16 = 0xFEFF;
pub const RESTRICTED_MEMORY_SIZE: usize =
    (RESTRICTED_MEMORY_END - RESTRICTED_MEMORY_START + 1) as usize;

pub const IO_START: u16 = 0xFF00;
pub const IO_END: u16 = 0xFF7F;
pub const IO_SIZE: usize = (IO_END - IO_START + 1) as usize;

pub const HRAM_START: u16 = 0xFF80;
pub const HRAM_END: u16 = 0xFFFE;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_START + 1) as usize;

pub const IE_REGISTER: u16 = 0xFFFF;

pub enum MemRegion {
    RomBank0,
    RomBank1,
    Vram,
    Exram,
    Wram0,
    Wram1,
    EchoRam,
    Oam,
    Restricted,
    Io,
    Hram,
    Ie,
}

// impl Mmu {
//     fn map_address(&self, addr: u16) -> (MemRegion, u16){
//         match addr {
//             RomBank0,
//             RomBank1,
//             Vram,
//             Exram,
//             Wram0,
//             Wram1,
//             EchoRam,
//             Oam,
//             Restricted,
//             Io,
//             Hram,
//             Ie,
//         }
//     } 
// }