use super::Cpu;
use crate::registers;
use registers::Flag;
use registers::{R8, R16};

// Each ALU op has an 8-bit and 16-bit version.
// ADD also has an extra 16-but op (ADD SP, e8)

// 8bit (This one is private - should not be called directly) :
// fn OP_a_u8(&mut self, value: u8) {
// ...
// }

// 16bit:
// pub fn OP_a_r16(&mut self, value: u16) {
// ...
// }

// 8-bit ALU ops generally have three extra associated functions
// (INC and DEC are a bit different, but similar):

// pub fn {OPNAME}_a_r8(&mut self, r8: R8) {
//     let value = self.reg.get(r8);
//     self.{OPNAME}_a_u8(value);
// }

// pub fn {OPNAME}_a_at_hl(&mut self) {
//     let hl = self.reg.get16(R16::HL);
//     let value = self.mmu.readbyte(hl);
//     self.{OPNAME}_a_u8(value);
// }

// pub fn {OPNAME}P_a_n8(&mut self) {
//     let n8 = self.fetch_byte();
//     self.{OPNAME}_a_u8(n8);
// }

impl Cpu {
    // ADD
    fn add_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let sum = ra as u16 + value as u16;
        let result = sum as u8;

        self.reg.set(R8::A, result);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, (ra & 0xF) + (value & 0xF) > 0xF);
        self.reg.set_flag(Flag::C, sum > 0xFF);
    }

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

    pub fn add_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.add_a_u8(value);
    }

    pub fn add_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.add_a_u8(value);
    }

    pub fn add_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.add_a_u8(n8);
    }

    // ADC
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

    pub fn adc_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.adc_a_u8(value);
    }

    pub fn adc_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.adc_a_u8(value);
    }

    pub fn adc_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.adc_a_u8(n8);
    }

    // SUB
    fn sub_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(Flag::C, ra < value);

        self.reg.set(R8::A, result);
    }

    pub fn sub_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.sub_a_u8(value);
    }

    pub fn sub_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.sub_a_u8(value);
    }

    pub fn sub_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.sub_a_u8(n8);
    }

    // SBC
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

    pub fn sbc_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.sbc_a_u8(value);
    }

    pub fn sbc_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.sbc_a_u8(value);
    }

    pub fn sbc_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.sbc_a_u8(n8);
    }

    // AND
    fn and_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra & value;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, true);
        self.reg.set_flag(Flag::C, false);

        self.reg.set(R8::A, result);
    }

    pub fn and_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.and_a_u8(value);
    }

    pub fn and_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.and_a_u8(value);
    }

    pub fn and_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.and_a_u8(n8);
    }

    // OR
    fn or_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra | value;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, false);

        self.reg.set(R8::A, result);
    }

    pub fn or_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.or_a_u8(value);
    }

    pub fn or_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.or_a_u8(value);
    }

    pub fn or_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.or_a_u8(n8);
    }

    // XOR
    fn xor_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra ^ value;

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, false);
        self.reg.set_flag(Flag::C, false);

        self.reg.set(R8::A, result);
    }

    pub fn xor_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.xor_a_u8(value);
    }

    pub fn xor_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.xor_a_u8(n8);
    }

    pub fn xor_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.xor_a_u8(value);
    }

    // CP
    fn cp_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(Flag::C, ra < value);
    }

    pub fn cp_a_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        self.cp_a_u8(value);
    }

    pub fn cp_a_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        self.cp_a_u8(value);
    }

    pub fn cp_a_n8(&mut self) {
        let n8 = self.fetch_byte();
        self.cp_a_u8(n8);
    }

    // INC
    fn inc_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, false);
        self.reg.set_flag(Flag::H, ((value & 0x0F) + 1) > 0x0F);
        // Carry flag untouched

        result
    }

    pub fn inc_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        let result = self.inc_u8(value);

        self.reg.set(r8, result);
    }

    pub fn inc_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        let result = self.inc_u8(value);

        self.mmu.writebyte(hl, result);
    }

    // DEC
    fn dec_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);

        self.reg.set_flag(Flag::Z, result == 0);
        self.reg.set_flag(Flag::N, true);
        self.reg.set_flag(Flag::H, (value & 0x0F) == 0);
        // Carry flag untouched

        result
    }

    pub fn dec_r8(&mut self, r8: R8) {
        let value = self.reg.get(r8);
        let result = self.dec_u8(value);

        self.reg.set(r8, result);
    }

    pub fn dec_at_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);
        let result = self.dec_u8(value);

        self.mmu.writebyte(hl, result);
    }
}
