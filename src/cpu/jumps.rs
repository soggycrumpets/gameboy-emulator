use super::*;

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
        }
    }

    // JR
    fn jr(&mut self, n8: u8) {
        let e8 = n8 as i8; // convert u8 to relative address
        let pc = self.reg.get16(R16::PC);
        let new_addr = ((pc as i32) + e8 as i32) as u16;
        self.reg.set16(R16::PC, new_addr)
    }

    pub fn jr_e8(&mut self) {
        let n8 = self.fetch_byte();
        self.jr(n8);
    }

    pub fn jr_cc_e8(&mut self, flag: Flag, expect: bool) {
        let n8 = self.fetch_byte();

        if expect == self.reg.get_flag(flag) {
            self.jr(n8);
        }
    }

    // CALL
    fn call_u16(&mut self, addr: u16) {
        self.push_r16(R16::PC);
        self.jp_u16(addr);    
    }

    pub fn call_a16(&mut self) {
        let a16 = self.fetch_word();
        self.call_u16(a16);
    }

    pub fn call_cc_a16(&mut self, flag: Flag, expect: bool) {
        let a16 = self.fetch_word();

        if expect == self.reg.get_flag(flag) {
            self.call_u16(a16);
        }
    }

    // RET
    pub fn ret(&mut self) {
        self.pop_r16(R16::PC);
    }

    pub fn ret_cc(&mut self, flag: Flag, expect: bool) {
        if expect == self.reg.get_flag(flag) {
            self.ret();
        }
    }
}
