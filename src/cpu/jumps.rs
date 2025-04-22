use super::*;

// Note: All instructions with checked conditions will return the
// number of extra clock tick cycles they took (actual_cycles - min_cycles)
const JP_CC_EXTRA_T_CYCLES: u8 = 4;
const CALL_CC_EXTRA_T_CYCLES: u8 = 12;
const RET_CC_EXTRA_T_CYCLES: u8 = CALL_CC_EXTRA_T_CYCLES;

impl Cpu {
    // JP
    fn jp_u16(&mut self, addr: u16) {
        self.reg.set16(R16::PC, addr);
    }

    pub fn jp_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        self.jp_u16(hl);
    }

    pub fn jp_a16(&mut self) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            4 => (),
            // Read a16 lower byte
            3 => self.word_buf_low = self.fetch_byte(),
            // Read a16 upper byte and jump to a16
            2 => {
                self.word_buf_high = self.fetch_byte();
                let addr = self.get_word_buf();
                self.jp_u16(addr);
            }
            // Internal
            1 => (),
            _ => unreachable!(),
        }
    }

    pub fn jp_cc_a16(&mut self, flag: Flag, expect: bool) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            4 => (),
            // Read a16 lower byte
            3 => self.word_buf_low = self.fetch_byte(),
            // Read a16 upper byte and jump to a16
            2 => {
                self.word_buf_high = self.fetch_byte();
                let addr = self.get_word_buf();
                if expect == self.reg.get_flag(flag) {
                    self.jp_u16(addr);
                } else {
                    self.instruction_t_cycles_remaining -= JP_CC_EXTRA_T_CYCLES;
                }
            }
            // Internal
            1 => (),
            _ => unreachable!(),
        }
    }

    // JR
    fn jr(&mut self, byte: u8) {
        // This instruction interprets the byte as signed
        let e8 = byte as i8;
        let pc = self.reg.get16(R16::PC);
        let new_addr = ((pc as i32) + e8 as i32) as u16;
        self.reg.set16(R16::PC, new_addr)
    }

    pub fn jr_e8(&mut self) {
        let byte = self.fetch_byte();
        self.jr(byte);
    }

    pub fn jr_cc_e8(&mut self, flag: Flag, expect: bool) {
        let byte = self.fetch_byte();

        if expect == self.reg.get_flag(flag) {
            self.jr(byte);
        } else {
            self.instruction_t_cycles_remaining -= JP_CC_EXTRA_T_CYCLES;
        }
    }

    // CALL
    // The cycle timings and actual function line up perfectly with PUSH, so I'm reusing it.
    pub fn rst_vec(&mut self, addr: u16) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            4 => (),
            // Internal
            3 => (),
            // Write the high byte to memory
            2 => self.push_r16(R16::PC),
            // Write the low byte to memory, then jump
            1 => {
                self.push_r16(R16::PC);
                self.jp_u16(addr);
            }
            _ => unreachable!(),
        }
    }
    
    // Currently, this function is only used by interrupts.
    pub fn rst_vec_instant(&mut self, addr: u16) {
        self.push_r16_instant(R16::PC);
        self.jp_u16(addr);
    }

    pub fn call_a16(&mut self) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            6 => (),
            // Read the low byte of a16
            5 => self.word_buf_low = self.fetch_byte(),
            // Read the high byte of a16
            4 => self.word_buf_high = self.fetch_byte(),
            // Internal
            3 => (),
            // Write high PC to SP
            2 => {
                let word = self.get_word_buf();
                self.rst_vec(word);
            }
            // Write low PC to SP
            1 => {
                let word = self.get_word_buf();
                self.rst_vec(word);
            }
            _ => unreachable!(),
        }
    }

    pub fn call_cc_a16(&mut self, flag: Flag, expect: bool) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            6 => (),
            // Read the low byte of a16
            5 => self.word_buf_low = self.fetch_byte(),
            // Read the high byte of a16 and check condition
            4 => {
                self.word_buf_high = self.fetch_byte();
                let word = self.get_word_buf();
                if expect != self.reg.get_flag(flag) {
                    self.instruction_t_cycles_remaining -= CALL_CC_EXTRA_T_CYCLES;
                }
            }
            // Internal
            3 => (),
            // Write high PC to SP and check condition
            2 => {
                let word = self.get_word_buf();
                self.rst_vec(word);
            }
            // Write low PC to SP
            1 => {
                let word = self.get_word_buf();
                self.rst_vec(word);
            }
            _ => unreachable!(),
        }
    }

    // RET
    // RET is just POP SP, with slightly different timings
    pub fn ret(&mut self) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            4 => (),
            // Read the low byte from memory
            3 => {
                let sp = self.reg.get16(R16::SP);
                let low_byte = self.read_byte(sp);

                self.reg.set16_low(R16::PC, low_byte);
                self.reg.set16(R16::SP, sp.wrapping_add(1));
            }
            // Read the high byte from memory
            2 => {
                let sp = self.reg.get16(R16::SP);
                let high_byte = self.read_byte(sp);

                self.reg.set16_high(R16::PC, high_byte);
                self.reg.set16(R16::SP, sp.wrapping_add(1));
            }
            // Internal
            1 => (),
            _ => unreachable!(),
        }
    }

    pub fn ret_cc(&mut self, flag: Flag, expect: bool) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            5 => (),
            // Internal / Check condition
            4 => {
                if expect != self.reg.get_flag(flag) {
                    self.instruction_t_cycles_remaining -= RET_CC_EXTRA_T_CYCLES;
                }
            }
            // Read the low byte from memory
            3 => self.ret(),
            // Read the high byte from memory
            2 => self.ret(),
            // Internal
            1 => (),
            _ => unreachable!(),
        }
    }

    pub fn reti(&mut self) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            4 => (),
            // Read SP lower
            3 => self.ret(),
            // Read SP upper and enable interrupts
            2 => {
                self.ret();
                self.ei();
            }
            // Internal
            1 => (),
            _ => unreachable!(),
        }
    }
}