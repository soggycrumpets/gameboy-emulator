use super::Mmu;

// ----- Memory Regions -----

// Ranges are INCLUSIVE

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
pub const RESTRICTED_MEM_START: u16 = 0xFEA0;
pub const RESTRICTED_MEM_END: u16 = 0xFEFF;
pub const RESTRICTED_MEM_SIZE: usize = (RESTRICTED_MEM_END - RESTRICTED_MEM_START + 1) as usize;

pub const IO_START: u16 = 0xFF00;
pub const IO_END: u16 = 0xFF7F;
pub const IO_SIZE: usize = (IO_END - IO_START + 1) as usize;

pub const HRAM_START: u16 = 0xFF80;
pub const HRAM_END: u16 = 0xFFFE;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_START + 1) as usize;

pub const IE_REGISTER: u16 = 0xFFFF;

// ----- Register Addresses -----

// Timer registers
pub const DIV_REGISTER_ADDR: u16 = 0xFF04;
pub const TIMA_REGISTER_ADDR: u16 = 0xFF05;
pub const TMA_REGISTER_ADDR: u16 = 0xFF06;
pub const TAC_REGISTER_ADDR: u16 = 0xFF07;

// Interrupt registers
const TIMER_INTERRUPT_REGISTER_ADDR: u16 = 0x0050;


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

// Return the memory region that the address is in, and an address
//  offset by that region's start address.
// The offset address can be used to directly indexing the array 
//  that represents that region of memory
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