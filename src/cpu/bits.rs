use super::*;

pub enum BitshiftOp {
    Rl,
    Rlc,
    Rr,
    Rrc,
    Sla,
    Sra,
    Srl,
    Swap,
}

pub enum BitflagOp {
    Bit,
    Res,
    Set,
}

impl Cpu {
    // These functions are the public interface for all bitops
    pub fn bitshift_r8(&mut self, op: BitshiftOp, r8: R8) {
        let bits = self.reg.get(r8);
        let result = self.bitshift_u8(op, bits);

        self.reg.set(r8, result);
    }

    pub fn bitshift_at_hl(&mut self, op: BitshiftOp) {
        let bits = self.read_at_hl();
        let result = self.bitshift_u8(op, bits);

        self.write_at_hl(result);
    }

    pub fn bitflag_u3_r8(&mut self, op: BitflagOp, bit: u8, r8: R8) {
        let bits = self.reg.get(r8);
        let result = self.bitflag_u3_u8(op, bit, bits);
        self.reg.set(r8, result);
    }

    pub fn bitflag_u3_at_hl(&mut self, op: BitflagOp, bit: u8) {
        let bits = self.read_at_hl();
        let result = self.bitflag_u3_u8(op, bit, bits);

        self.write_at_hl(result);
    }

    // Special unprefixed operations
    pub fn rlca(&mut self) {
        self.bitshift_r8(BitshiftOp::Rlc, R8::A);
        self.reg.set_flag(Flag::Z, false);
    }

    pub fn rrca(&mut self) {
        self.bitshift_r8(BitshiftOp::Rrc, R8::A);
        self.reg.set_flag(Flag::Z, false);
    }

    pub fn rla(&mut self) {
        self.bitshift_r8(BitshiftOp::Rl, R8::A);
        self.reg.set_flag(Flag::Z, false);
    }

    pub fn rra(&mut self) {
        self.bitshift_r8(BitshiftOp::Rr, R8::A);
        self.reg.set_flag(Flag::Z, false);
    }

    // This function maps bitshifts to their functions
    fn bitshift_u8(&mut self, op: BitshiftOp, bits: u8) -> u8 {
        match op {
            BitshiftOp::Rl => self.rl_u8(bits),
            BitshiftOp::Rlc => self.rlc_u8(bits),
            BitshiftOp::Rr => self.rr_u8(bits),
            BitshiftOp::Rrc => self.rrc_u8(bits),
            BitshiftOp::Sla => self.sla_u8(bits),
            BitshiftOp::Sra => self.sra_u8(bits),
            BitshiftOp::Srl => self.srl_u8(bits),
            BitshiftOp::Swap => self.swap_u8(bits),
        }
    }

    // This one matches bitflags to their functions
    fn bitflag_u3_u8(&mut self, op: BitflagOp, bit: u8, bits: u8) -> u8 {
        match op {
            BitflagOp::Bit => {
                // This one doesn't change the bits it targets
                // So return the same bits
                self.bit_u3_u8(bit, bits);
                bits
            }
            BitflagOp::Res => self.res_u3_u8(bit, bits),
            BitflagOp::Set => self.set_u3_u8(bit, bits),
        }
    }

    // All bit shift functions set flags in the same way
    // The only difference is which bit they shift to the carry flag
    fn set_shift_flags(&mut self, result: u8, carry: bool) {
        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, carry);
    }

    // ----- Bitshift Instructions -----
    
    // RL
    fn rl_u8(&mut self, bits: u8) -> u8 {
        let c = self.reg.get_flag(Flag::C);
        let upper_bit = (bits & (1 << 7)) != 0;

        let result = (bits << 1) | (c as u8);

        self.set_shift_flags(result, upper_bit);

        result
    }

    // RLC
    fn rlc_u8(&mut self, bits: u8) -> u8 {
        let upper_bit = (bits & (1 << 7)) != 0;

        let result = (bits << 1) | (upper_bit as u8);

        self.set_shift_flags(result, upper_bit);

        result
    }

    // RR
    fn rr_u8(&mut self, bits: u8) -> u8 {
        let c = self.reg.get_flag(Flag::C);
        let lower_bit = (bits & 1) != 0;

        let result = (bits >> 1) | ((c as u8) << 7);

        self.set_shift_flags(result, lower_bit);

        result
    }

    // RRC
    fn rrc_u8(&mut self, bits: u8) -> u8 {
        let lower_bit = (bits & 1) != 0;

        let result = (bits >> 1) | ((lower_bit as u8) << 7);

        self.set_shift_flags(result, lower_bit);

        result
    }

    // SLA
    fn sla_u8(&mut self, bits: u8) -> u8 {
        let upper_bit = (bits & (1 << 7)) != 0;

        let result = bits << 1;

        self.set_shift_flags(result, upper_bit);

        result
    }

    // SRA
    fn sra_u8(&mut self, bits: u8) -> u8 {
        let lower_bit = (bits & 1) != 0;
        let upper_bit = (bits & (1 << 7)) != 0;

        let result = (bits >> 1) | ((upper_bit as u8) << 7);

        self.set_shift_flags(result, lower_bit);

        result
    }

    // SRL
    fn srl_u8(&mut self, bits: u8) -> u8 {
        let lower_bit = (bits & 1) != 0;

        let result = bits >> 1;

        self.set_shift_flags(result, lower_bit);

        result
    }

    // SWAP
    fn swap_u8(&mut self, bits: u8) -> u8 {
        let upper_bits = bits & 0xF0;
        let lower_bits = bits & 0x0F;

        let result = (upper_bits >> 4) | (lower_bits << 4);

        // The carry
        self.set_shift_flags(result, false);

        result
    }

    // ----- Bitflag Instructions -----

    // BIT
    fn bit_u3_u8(&mut self, bit_position: u8, bits: u8) {
        let result = (bits & (1 << bit_position)) == 0;

        self.reg.set_flag(Flag::Z, result);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, true);
        // Carry flag untouched
    }

    // RES
    fn res_u3_u8(&mut self, bit_position: u8, bits: u8) -> u8 {
        bits & !(1 << bit_position)
    }

    // SET
    fn set_u3_u8(&mut self, bit_position: u8, bits: u8) -> u8 {
        bits | (1 << bit_position)
    }
}
