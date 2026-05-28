/// This value is written to the serial control address to signal an output
const TRANSFER_REQUESTED_VALUE: u8 = 0x81;
/// Attempts to access an unavailable region of memory typically retusn all high bits
const GARBAGE_VALUE: u8 = 0xFF;

use super::*;
impl Mmu {
    /// Read a byte from memory. There are many side-effects and special cases that determine
    /// how exactly the read is processed.
    pub fn read_byte(&self, addr: u16) -> u8 {
        // TODO: Maybe inline or remove one or both of these functions; this is a very hot path
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
                STAT_ADDR => self.io[index] | 0b_1000_0000, // Upper bit is unused
                P1_ADDR => self.read_byte_p1(),
                IF_ADDR => self.io[index] | 0b_1110_0000, // Upper 3 bits are unused
                _ => self.io[index],
            },
            M::Hram => self.hram[index],
            M::Ie => self.ie,
        }
    }

    /// Write a byte to memory. There are many side-effects and special cases that determine
    /// how exactly the read is processed.
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        let mem_region = map_region(addr);
        let index = map_addr(addr);

        // Blargg's test roms use the serial output registers to output test results
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
            M::Io => match addr {
                DIV_ADDR => self.write_byte_div(),
                TMA_ADDR => self.write_byte_tma(byte),
                TAC_ADDR => self.write_byte_tac(byte),
                TIMA_ADDR => self.write_byte_tima(byte),
                DMA_ADDR => self.start_dma_transfer(byte),
                LY_ADDR => (),                                     // Read-only
                STAT_ADDR => self.io[index] = byte & 0b_1111_1000, // Bottom 3 bits are read-only
                IF_ADDR => self.io[index] = byte | 0b_1110_0000,   // Top 3 bits are always 1
                P1_ADDR => self.write_byte_p1(byte),
                _ => self.io[index] = byte,
            },
            M::Hram => self.hram[index] = byte,
            M::Ie => self.ie = byte,
        };
    }

    /// Read one byte from memory, bypassing all of the special cases
    /// and side-effects of the standard read_byte function. Use this with caution!
    pub fn read_byte_override(&self, addr: u16) -> u8 {
        let mem_region = map_region(addr);
        let index = map_addr(addr);

        use MemRegion as M;
        match mem_region {
            M::RomBank0 => self.rom_bank_00[index],
            M::RomBank1 => self.rom_bank_01[index],
            M::Vram => self.vram[index],
            M::Exram => self.exram[index],
            M::Wram0 => self.wram_0[index],
            M::Wram1 => self.wram_1[index],
            M::EchoRam => self.read_byte_override(addr - ECHO_OFFSET),
            M::Oam => self.oam[index],
            M::Restricted => self.restricted_memory[index],
            M::Io => self.io[index],
            M::Hram => self.hram[index],
            M::Ie => self.ie,
        }
    }

    /// Write one byte from memory, bypassing all of the special cases
    /// and side-effects of the standard read_byte function. Use this with caution!
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
            M::EchoRam => self.write_byte_override(addr - ECHO_OFFSET, byte),
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

    /// The Gameboy uses 2 selector bits, which allows it to map 8 buttons to only 4 bits
    fn read_byte_p1(&self) -> u8 {
        let mut byte = self.read_byte_override(P1_ADDR);

        let select_buttons = get_bit(byte, SELECT_BUTTONS_BIT as u8);
        let select_dpad = get_bit(byte, SELECT_DPAD_BIT as u8);

        if !select_buttons {
            set_bit(&mut byte, START_DOWN_BIT as u8, !self.buttons.start);
            set_bit(&mut byte, SELECT_UP_BIT as u8, !self.buttons.select);
            set_bit(&mut byte, B_LEFT_BIT as u8, !self.buttons.b);
            set_bit(&mut byte, A_RIGHT_BIT as u8, !self.buttons.a);
        } else if !select_dpad {
            set_bit(&mut byte, START_DOWN_BIT as u8, !self.buttons.down);
            set_bit(&mut byte, SELECT_UP_BIT as u8, !self.buttons.up);
            set_bit(&mut byte, B_LEFT_BIT as u8, !self.buttons.left);
            set_bit(&mut byte, A_RIGHT_BIT as u8, !self.buttons.right);
        } else {
            byte |= 0x0F;
        }

        byte
    }

    // Only the select bits, 4 and 5, are writable
    fn write_byte_p1(&mut self, mut byte_to_write: u8) {
        let mut byte = self.read_byte_override(P1_ADDR);

        let mask = 0b_1100_1111;

        byte &= mask;
        byte_to_write &= !mask;
        byte |= byte_to_write;

        self.write_byte_override(P1_ADDR, byte);
    }
}
