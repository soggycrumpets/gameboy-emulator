use super::Mmu;

// ----- Memory ranges are INCLUSIVE -----

const ROM_BANK_0_START: u16 = 0x0000;
const ROM_BANK_0_END: u16 = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = (ROM_BANK_0_END - ROM_BANK_0_START + 1) as usize;

// Switchable bank
const ROM_BANK_1_START: u16 = 0x4000;
const ROM_BANK_1_END: u16 = 0x7FFF;
pub const ROM_BANK_1_SIZE: usize = (ROM_BANK_1_END - ROM_BANK_1_START + 1) as usize;

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16 = 0x9FFF;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;

// Cartridge's external RAM
const EXRAM_START: u16 = 0xA000;
const EXRAM_END: u16 = 0xBFFF;
pub const EXRAM_SIZE: usize = (EXRAM_END - EXRAM_START + 1) as usize;

const WRAM_0_START: u16 = 0xC000;
const WRAM_0_END: u16 = 0xCFFF;
pub const WRAM_0_SIZE: usize = (WRAM_0_END - WRAM_0_START + 1) as usize;

// Sitchable bank 1-7 in CGB mode
const WRAM_1_START: u16 = 0xD000;
const WRAM_1_END: u16 = 0xDFFF;
pub const WRAM_1_SIZE: usize = (WRAM_1_END - WRAM_1_START + 1) as usize;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;
pub const ECHO_RAM_SIZE: usize = (ECHO_RAM_END - ECHO_RAM_START + 1) as usize;

const OAM_START: u16 = 0xFE00;
const OAM_END: u16 = 0xFE9F;
pub const OAM_SIZE: usize = (OAM_END - OAM_START + 1) as usize;

// Prohibited area of memory
const RESTRICTED_MEM_START: u16 = 0xFEA0;
const RESTRICTED_MEM_END: u16 = 0xFEFF;
pub const RESTRICTED_MEM_SIZE: usize = (RESTRICTED_MEM_END - RESTRICTED_MEM_START + 1) as usize;

const IO_START: u16 = 0xFF00;
const IO_END: u16 = 0xFF7F;
pub const IO_SIZE: usize = (IO_END - IO_START + 1) as usize;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_START + 1) as usize;

const IE_REGISTER: u16 = 0xFFFF;

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

pub fn map_address(addr: u16) -> (MemRegion, u16) {
    use MemRegion as M;
    let (region, start_addr) = match addr {
        ROM_BANK_0_START..=ROM_BANK_0_END => (M::RomBank0, ROM_BANK_0_START),
        ROM_BANK_1_START..=ROM_BANK_1_END => (M::RomBank1, ROM_BANK_1_START),
        VRAM_START..=VRAM_END => (M::Vram, VRAM_START),
        EXRAM_START..=EXRAM_END => (M::Exram, EXRAM_START),
        WRAM_0_START..=WRAM_0_END => (M::Wram0, WRAM_0_START),
        WRAM_1_START..=WRAM_1_END => (M::Wram1, WRAM_1_START),
        ECHO_RAM_START..=ECHO_RAM_END => (M::EchoRam, ECHO_RAM_START),
        OAM_START..=OAM_END => (M::Oam, OAM_START),
        RESTRICTED_MEM_START..=RESTRICTED_MEM_END => (M::Restricted, RESTRICTED_MEM_START),
        IO_START..=IO_END => (M::Io, IO_START),
        HRAM_START..=HRAM_END => (M::Hram, HRAM_START),
        IE_REGISTER => (M::Ie, IE_REGISTER),
    };

    (region, addr - start_addr)
}