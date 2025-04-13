use super::Cpu;
use crate::registers;
use registers::Flag;
use registers::{R8, R16};

impl Cpu {
    pub fn di(&mut self) {
        self.ime = false;
        self.ime_pending = false;
    }

    pub fn ei(&mut self) {
        self.ime_pending = true;
    }

    pub fn cpl(&mut self) {
        let ra = self.reg.get(R8::A);

        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, true);

        self.reg.set(R8::A, !ra);
    }

    pub fn scf(&mut self) {
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, true);
    }

    pub fn ccf(&mut self) {
        let carry = self.reg.get_flag(Flag::C);

        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, !carry);
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
                carry = true;
            }
            ra.wrapping_sub(adjustment)
        } else {
            if h || ((ra & 0xF) > 0x9) {
                adjustment += 0x6;
            }
            if c || (ra > 0x99) {
                adjustment += 0x60;
                carry = true;
            }
            ra.wrapping_add(adjustment)
        };

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, carry);

        self.reg.set(R8::A, result);
    }
}
