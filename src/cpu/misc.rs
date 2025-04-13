use super::Cpu;
use crate::registers;
use registers::Flag;
use registers::{R8, R16};

impl Cpu {
    pub fn di(&mut self) {
        self.ime = false;
    }

    pub fn ei(&mut self) {
        self.ime = true;
    }

    pub fn daa(&mut self) {
        let ra = self.reg.get(R8::A);
        let n = self.reg.get_flag(Flag::N);
        let h = self.reg.get_flag(Flag::H);
        let c = self.reg.get_flag(Flag::C);
        let mut adjustment: u8 = 0;
        let mut carry = false;
        let result = if n {
            if h {
                adjustment += 0x6;
            }
            if c {
                adjustment += 0x60;
            }
            ra.wrapping_sub(adjustment)
        } else {
            if h || ((ra & 0xF) > 0x9) {
                adjustment += 0x6;
            }
            if c || (ra > 0x99) {
                adjustment += 0x60;
            }
            ra.wrapping_add(adjustment)
        };

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, carry);

        self.reg.set(R8::A, result);
    }
}
