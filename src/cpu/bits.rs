use super::*;

pub enum Bitop {
    RL,
    RLC,
    RR,
    RRC,
    SLA,
    SRA,
    SWAP,
    SRL,
    BIT,
    RES,
    SET,
}

impl Cpu {
    // This function matches bitops to functions
    fn bitop_u8(&mut self, op: Bitop, bits: u8) -> u8 {
        match op {
            Bitop::RL => self.rl_u8(bits),
            Bitop::RLC => self.rlc_u8(bits),
            Bitop::RR => self.rr_u8(bits),
            Bitop::RRC => self.rrc_u8(bits),
            Bitop::SLA => self.sla_u8(bits),
            _ => todo!("Add the rest of the bitops"),
        }
    }

    // These two functions are the public interface for all bitops
    pub fn bitop_r8(&mut self, op: Bitop, r8: R8) {
        let bits = self.reg.get(r8);
        let result = self.bitop_u8(op, bits);

        self.reg.set(r8, result);
    }

    pub fn bitop_at_hl(&mut self, op: Bitop) {
        let hl = self.reg.get16(R16::HL);
        let bits = self.mmu.readbyte(hl);
        let result = self.bitop_u8(op, bits);

        self.mmu.writebyte(hl, result);
    }

    // RL
    fn rl_u8(&mut self, bits: u8) -> u8 {
        let c = self.reg.get_flag(Flag::C);
        let upper_bit = (bits & (1 << 7)) != 0;

        let result = (bits << 1) | (c as u8);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, upper_bit);

        result
    }

    // RLC
    fn rlc_u8(&mut self, bits: u8) -> u8 {
        let upper_bit = (bits & (1 << 7)) != 0;

        let result = (bits << 1) | (upper_bit as u8);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, upper_bit);

        result
    }

    // RR
    fn rr_u8(&mut self, bits: u8) -> u8 {
        let c = self.reg.get_flag(Flag::C);
        let lower_bit = (bits & 1) != 0;

        let result = (bits >> 1) | ((c as u8) << 7);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, lower_bit);

        result
    }

    // RRC
    fn rrc_u8(&mut self, bits: u8) -> u8 {
        let lower_bit = (bits & 1) != 0;

        let result = (bits >> 1) | ((lower_bit as u8) << 7);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, lower_bit);

        result
    }

    // SLA
    fn sla_u8(&mut self, bits: u8) -> u8 {
        let upper_bit = (bits & (1 << 7)) != 0;

        let result = bits << 1;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, upper_bit);

        result
    }
}
