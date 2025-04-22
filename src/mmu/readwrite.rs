// ! This value is written to 
const TRANSFER_REQUESTED_VALUE: u8 = 0x81;
// ! Attempts to access an unavailable region of memory typically retusn all high bits
const GARBAGE_VALUE: u8 = 0xFF;

use super::*;
impl Mmu {
    //! Memory regions are all treated separately, and lots of regions and addresses
    //! have special rules that determine what happens when a read or write is done.
    pub fn read_byte(&self, addr: u16) -> u8 {
        let mem_region = map_region(addr);
        let index = map_addr(addr);

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
                IF_ADDR => self.io[index] | 0b_1110_0000, // Upper 3 bits always read high
                _ => self.io[index],
            },
            M::Hram => self.hram[index],
            M::Ie => self.ie,
        }
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let mem_region= map_region(addr);
        let index = map_addr(addr);

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

    /// This function bypasses all of the special conditions and side-effects of the 
    /// standard read_byte function. Use this carefully!
    pub fn read_byte_override(&self, addr: u16) -> u8 {
        let mem_region= map_region(addr);
        let index = map_addr(addr);

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

   

    /// This function bypasses all of the special conditions and side-effects of the 
    /// standard write_byte function. Use this carefully!
    pub fn write_byte_override(&mut self, addr: u16, byte: u8) {
        let index = map_addr(addr);
        let region = map_region(addr);

        use MemRegion as M;
        match region {
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

    /// Read a two-byte value to memory, in little-endian order
    pub fn read_word(&self, addr: u16) -> u16 {
        let lowbyte = self.read_byte(addr);
        let highbyte = self.read_byte(addr + 1);
        lowbyte as u16 | ((highbyte as u16) << 8)
    }

    /// Write a two-byte value to memory, in little-endian order
    pub fn write_word(&mut self, addr: u16, word: u16) {
        let lowbyte = word as u8;
        let highbyte = (word >> 8) as u8;
        self.write_byte(addr, lowbyte);
        self.write_byte(addr + 1, highbyte);
    }

    /// An interrupt is requested by setting a specific bit in the IF register
    pub fn request_interrupt(&mut self, interrupt_bit: u8) {
        let mut byte = self.read_byte(IF_ADDR);
        set_bit(&mut byte, interrupt_bit, true);
        self.write_byte(IF_ADDR, byte);
    }

}
