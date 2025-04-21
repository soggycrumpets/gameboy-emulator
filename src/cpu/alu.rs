use super::*;
use registers::{Flag, R8, R16};

// These are all of the main operations that the ALU can perform.
// INC, DEC, CPL, SCF, CCF, DAA, and all of the 16-bit ops have
// have their own interface functions

pub enum AluBinary {
    Add,
    Adc,
    Sub,
    Sbc,
    And,
    Xor,
    Or,
    Cp,
}

pub enum AluUnary {
    Inc,
    Dec,
}

impl Cpu {
    // These functions map the most common ALU operations to their functions.
    // This one is for binary operations:
    fn alu_a_u8(&mut self, op: AluBinary, value: u8) {
        match op {
            AluBinary::Add => self.add_a_u8(value),
            AluBinary::Adc => self.adc_a_u8(value),
            AluBinary::Sub => self.sub_a_u8(value),
            AluBinary::Sbc => self.sbc_a_u8(value),
            AluBinary::And => self.and_a_u8(value),
            AluBinary::Xor => self.xor_a_u8(value),
            AluBinary::Or => self.or_a_u8(value),
            AluBinary::Cp => self.cp_a_u8(value),
        };
    }

    // This one is for unary operations:
    fn alu_u8(&mut self, op: AluUnary, value: u8) -> u8 {
        match op {
            AluUnary::Inc => self.inc_u8(value),
            AluUnary::Dec => self.dec_u8(value),
        }
    }

    // These are the ALU interface functions for 8-bit operations:
    pub fn alu_a_r8(&mut self, op: AluBinary, r8: R8) {
        let value = self.reg.get(r8);
        self.alu_a_u8(op, value);
    }

    pub fn alu_a_at_hl(&mut self, op: AluBinary) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            2 => (),
            // Read at HL
            1 => {
                let value = self.read_at_hl();
                self.alu_a_u8(op, value);
            }
            _ => unreachable!(),
        }
    }

    pub fn alu_a_n8(&mut self, op: AluBinary) {
        let value = self.fetch_byte();
        self.alu_a_u8(op, value);
    }

    pub fn alu_r8(&mut self, op: AluUnary, r8: R8) {
        let value = self.reg.get(r8);
        let result = self.alu_u8(op, value);

        self.reg.set(r8, result);
    }

    pub fn alu_at_hl(&mut self, op: AluUnary) {
        let value = self.read_at_hl();
        let result = self.alu_u8(op, value);

        self.write_at_hl(result);
    }

    // ----- ADD -----
    fn add_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let sum = ra as u16 + value as u16;
        let result = sum as u8;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, (ra & 0xF) + (value & 0xF) > 0xF);
        self.reg.set_flag(Flag::C, sum > 0xFF);

        self.reg.set(R8::A, result);
    }

    // 16-bit
    pub fn add_hl_r16(&mut self, r16: R16) {
        let hl = self.reg.get16(R16::HL);
        let value = self.reg.get16(r16);

        let result = hl.wrapping_add(value);

        // Zero flag untouched
        self.reg.set_flag(Flag::N, false);
        self.reg
            .set_flag(Flag::H, (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        self.reg
            .set_flag(Flag::C, (hl as u32) + (value as u32) > 0xFFFF);

        self.reg.set16(R16::HL, result);
    }

    // This contains the core functionality of add_sp_e8, as well as ld_hl_sp_e8 from load.rs
    pub fn calc_sp_plus_e8(&mut self) -> u16 {
        let sp = self.reg.get16(R16::SP);
        let n8 = self.fetch_byte();

        // Casting this way allows e8 to be negative if n8 is big enough (n8 > i8_MAX results in a negative)
        // Casting directly to an i16 would not change the sign
        let e8 = (n8 as i8) as i16;

        let result = (sp as i16).wrapping_add(e8) as u16;

        self.reg.set_flag(Flag::Z, false);
        self.reg.set_flag(Flag::N, false);
        self.reg
            .set_flag(Flag::H, (sp as u8 & 0x0F) + (n8 & 0x0F) > 0x0F);
        self.reg
            .set_flag(Flag::C, (sp & 0x00FF) + ((n8 as u16) & 0x00FF) > 0xFF);

        result
    }

    pub fn add_sp_e8(&mut self) {
        let result = self.calc_sp_plus_e8();
        self.reg.set16(R16::SP, result);
    }

    // ----- ADC -----
    fn adc_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);
        let carry = self.reg.get_flag(Flag::C) as u8;

        let sum = ra as u16 + carry as u16 + value as u16;
        let result = sum as u8;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg
            .set_flag(Flag::H, (ra & 0xF) + (value & 0xF) + carry > 0xF);
        self.reg.set_flag(Flag::C, sum > 0xFF);

        self.reg.set(R8::A, result);
    }

    // ----- SUB -----
    fn sub_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(Flag::C, ra < value);

        self.reg.set(R8::A, result);
    }

    // ----- SBC -----
    fn sbc_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);
        let carry = self.reg.get_flag(Flag::C) as u8;

        let result = ra.wrapping_sub(value).wrapping_sub(carry);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, true);
        self.reg
            .set_flag(Flag::H, (ra & 0xF) < ((value & 0xF) + carry));
        self.reg
            .set_flag(Flag::C, (ra as u16) < (value as u16 + carry as u16));

        self.reg.set(R8::A, result);
    }

    // ----- AND -----
    fn and_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra & value;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, true);
        self.reg.set_flag(Flag::C, false);

        self.reg.set(R8::A, result);
    }

    // ----- OR -----
    fn or_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra | value;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, false);

        self.reg.set(R8::A, result);
    }

    //  ----- XOR -----
    fn xor_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra ^ value;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, false);

        self.reg.set(R8::A, result);
    }

    // ----- CP -----
    fn cp_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(Flag::C, ra < value);
    }

    // ----- INC -----
    fn inc_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, ((value & 0x0F) + 1) > 0x0F);

        result
    }

    // 16-bit
    pub fn inc_r16(&mut self, r16: R16) {
        let value = self.reg.get16(r16);
        let result = value.wrapping_add(1);

        self.reg.set16(r16, result);
    }

    // ----- DEC -----
    fn dec_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, (value & 0x0F) == 0);

        result
    }

    // 16-bit
    pub fn dec_r16(&mut self, r16: R16) {
        let value = self.reg.get16(r16);
        let result = value.wrapping_sub(1);

        self.reg.set16(r16, result);
    }

    // ----- CPL -----
    pub fn cpl(&mut self) {
        let ra = self.reg.get(R8::A);

        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, true);

        self.reg.set(R8::A, !ra);
    }

    // -----  SCF -----
    pub fn scf(&mut self) {
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, true);
    }

    //  ----- CCF -----
    pub fn ccf(&mut self) {
        let carry = self.reg.get_flag(Flag::C);

        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, !carry);
    }

    //  ----- DAA -----
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
