use super::*;

impl Cpu {
    // JP

    pub fn jp_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        self.reg.set16(R16::PC, hl);
    }

    pub fn jp_a16(&mut self) {
        let n16 = self.fetch_word();
        self.reg.set16(R16::PC, n16);
    }

    pub fn jp_cc_a16(&mut self, flag: Flag, expect: bool) {
        let a16 = self.fetch_word();

        if expect == self.reg.get_flag(flag) {
            self.reg.set16(R16::PC, a16);
        }
    }

    // JR
    fn jr(&mut self, n8: u8) {
        let e8 = n8 as i8;
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
}
