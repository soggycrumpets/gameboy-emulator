//! For documentation on each of registers, check out the
//! [Pandocs](https://gbdev.io/pandocs/Hardware_Reg_List.html?highlight=hardware%20registers#hardware-registers)

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
pub const ECHO_OFFSET: u16 = ECHO_RAM_START - WRAM_0_START;

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

pub const IE_ADDR: u16 = 0xFFFF;

// ----- Register Addresses -----
// Input
pub const P1_ADDR: u16 = 0xFF00;

// Timer
pub const DIV_ADDR: u16 = 0xFF04;
pub const TIMA_ADDR: u16 = 0xFF05;
pub const TMA_ADDR: u16 = 0xFF06;
pub const TAC_ADDR: u16 = 0xFF07;

// PPU
pub const LCDC_ADDR: u16 = 0xFF40;
// - Bits within LCDC
pub const LCD_AND_PPU_ENABLE_BIT: u8 = 7;
pub const WINDOW_TILE_MAP_BIT: u8 = 6;
pub const WINDOW_ENABLE_BIT: u8 = 5;
pub const BG_AND_WINDOW_TILES_BIT: u8 = 4;
pub const BG_TILE_MAP_BIT: u8 = 3;
pub const OBJ_SIZE_BIT: u8 = 2;
pub const OBJ_ENABLE_BIT: u8 = 1;
pub const BG_AND_WINDOW_ENABLE_BIT: u8 = 0;
// -
pub const STAT_ADDR: u16 = 0xFF41;
// - Bits within STAT
pub const LYC_INT_SELECT_BIT: u8 = 6;
pub const MODE_2_INT_SELECT_BIT: u8 = 5;
pub const MODE_1_INT_SELECT_BIT: u8 = 4;
pub const MODE_0_INT_SELECT_BIT: u8 = 3;
pub const LY_EQUALS_LYC_BIT: u8 = 2;
//-
pub const LY_ADDR: u16 = 0xFF44;
pub const LYC_ADDR: u16 = 0xFF45;
pub const SCY_ADDR: u16 = 0xFF42;
pub const SCX_ADDR: u16 = 0xFF43;
pub const DMA_ADDR: u16 = 0xFF46;
pub const BGP_ADDR: u16 = 0xFF47;
pub const OBP0_ADDR: u16 = 0xFF48;
pub const OBP1_ADDR: u16 = 0xFF49;
pub const WY_ADDR: u16 = 0xFF4A;
pub const WX_ADDR: u16 = 0xFF4B;

// Interrupt (besides IE, which counts but is also its own memory region)
pub const IF_ADDR: u16 = 0xFF0F;
// - Bits within IF and IE
pub const VBLANK_INTERRUPT_BIT: u8 = 0;
pub const STAT_INTERRUPT_BIT: u8 = 1;
pub const TIMER_INTERRUPT_BIT: u8 = 2;
pub const SERIAL_INTERRUPT_BIT: u8 = 3;
pub const JOYPAD_INTERRUPT_BIT: u8 = 4;
// -
pub const VBLANK_INTERRUPT_HANDLER_ADDR: u16 = 0x0040;
pub const STAT_INTERRUPT_HANDLER_ADDR: u16 = 0x0048;
pub const TIMER_INTERRUPT_HANDLER_ADDR: u16 = 0x0050;
pub const SERIAL_INTERRUPT_HANDLER_ADDR: u16 = 0x0058;
pub const JOYPAD_INTERRUPT_HANDLER_ADDR: u16 = 0x0060;

// Serial transfer
pub const SC_ADDR: u16 = 0xFF02;
pub const SB_ADDR: u16 = 0xFF01;

// Audio
pub const NR_10_ADDR: u16 = 0xFF10;
pub const NR_11_ADDR: u16 = 0xFF11;
pub const NR_12_ADDR: u16 = 0xFF12;
pub const NR_13_ADDR: u16 = 0xFF13;
pub const NR_14_ADDR: u16 = 0xFF14;

pub const NR_21_ADDR: u16 = 0xFF16;
pub const NR_22_ADDR: u16 = 0xFF17;
pub const NR_23_ADDR: u16 = 0xFF18;
pub const NR_24_ADDR: u16 = 0xFF19;

pub const NR_30_ADDR: u16 = 0xFF1A;
pub const NR_31_ADDR: u16 = 0xFF1B;
pub const NR_32_ADDR: u16 = 0xFF1C;
pub const NR_33_ADDR: u16 = 0xFF1D;
pub const NR_34_ADDR: u16 = 0xFF1E;

pub const NR_41_ADDR: u16 = 0xFF20;
pub const NR_42_ADDR: u16 = 0xFF21;
pub const NR_43_ADDR: u16 = 0xFF22;
pub const NR_44_ADDR: u16 = 0xFF23;

pub const NR_50_ADDR: u16 = 0xFF24;
pub const NR_51_ADDR: u16 = 0xFF25;
pub const NR_52_ADDR: u16 = 0xFF26;

// ----- Other Important Addresses -----
pub const PROGRAM_START_ADDR: u16 = 0x0100;
pub const TOP_OF_STACK_ADDRESS: u16 = 0xFFFE;

#[derive(PartialEq)]
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

// Get an offset address relative to the start of that address's memory region
pub fn map_addr(addr: u16) -> usize {
    let start_addr = match addr {
        ROM_BANK_0_START..=ROM_BANK_0_END => ROM_BANK_0_START,
        ROM_BANK_1_START..=ROM_BANK_1_END => ROM_BANK_1_START,
        VRAM_START..=VRAM_END => VRAM_START,
        EXRAM_START..=EXRAM_END => EXRAM_START,
        WRAM_0_START..=WRAM_0_END => WRAM_0_START,
        WRAM_1_START..=WRAM_1_END => WRAM_1_START,
        ECHO_RAM_START..=ECHO_RAM_END => ECHO_RAM_START,
        OAM_START..=OAM_END => OAM_START,
        RESTRICTED_MEM_START..=RESTRICTED_MEM_END =>  RESTRICTED_MEM_START,
        IO_START..=IO_END => IO_START,
        HRAM_START..=HRAM_END =>  HRAM_START,
        IE_ADDR => IE_ADDR,
    };

    (addr - start_addr) as usize
}

/// Get the memory region of an address
pub fn map_region(addr: u16) -> MemRegion {
    use MemRegion as M;
    match addr {
        ROM_BANK_0_START..=ROM_BANK_0_END => M::RomBank0,
        ROM_BANK_1_START..=ROM_BANK_1_END => M::RomBank1,
        VRAM_START..=VRAM_END => M::Vram,
        EXRAM_START..=EXRAM_END => M::Exram,
        WRAM_0_START..=WRAM_0_END => M::Wram0,
        WRAM_1_START..=WRAM_1_END => M::Wram1,
        ECHO_RAM_START..=ECHO_RAM_END => M::EchoRam,
        OAM_START..=OAM_END => M::Oam,
        RESTRICTED_MEM_START..=RESTRICTED_MEM_END => M::Restricted,
        IO_START..=IO_END => M::Io,
        HRAM_START..=HRAM_END => M::Hram,
        IE_ADDR => M::Ie,
    }
}
