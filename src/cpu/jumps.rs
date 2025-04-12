use super::*;

impl Cpu {
    pub fn jp_n16(&mut self) {
        let n16 = self.fetch_word();
        self.reg.set16(R16::PC, n16);
    }

    pub fn jp_c_a16(&mut self) {
        let a16 = self.fetch_word();

        if self.reg.get_flag(Flag::C) {
            self.reg.set16(R16::PC, a16);
        }
    }
}
