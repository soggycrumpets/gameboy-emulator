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
        let a16 = self.fetch_word();
        self.jp_u16(a16);
    }

    pub fn jp_cc_a16(&mut self, flag: Flag, expect: bool) {
        let a16 = self.fetch_word();

        if expect == self.reg.get_flag(flag) {
            self.jp_u16(a16);
            self.instruction_t_cycles_remaining += JP_CC_EXTRA_T_CYCLES;
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
            self.instruction_t_cycles_remaining += 4;
        }
    }

    // CALL
    pub fn rst_vec(&mut self, addr: u16) {
        self.push_r16_instant(R16::PC);
        self.jp_u16(addr);    
    }

    pub fn call_a16(&mut self) {
        let word = self.fetch_word();
        self.rst_vec(word);
    }

    pub fn call_cc_a16(&mut self, flag: Flag, expect: bool) {
        let word = self.fetch_word();

        if expect == self.reg.get_flag(flag) {
            self.rst_vec(word);
            self.instruction_t_cycles_remaining += CALL_CC_EXTRA_T_CYCLES;
        }
    }

    // RET
    pub fn ret(&mut self) {
        self.pop_r16_instant(R16::PC);
    }

    pub fn ret_cc(&mut self, flag: Flag, expect: bool) {
        if expect == self.reg.get_flag(flag) {
            self.ret();
            self.instruction_t_cycles_remaining += RET_CC_EXTRA_T_CYCLES;
        }

    }

    pub fn reti(&mut self) {
        self.ret();
        self.ei();
    }
}