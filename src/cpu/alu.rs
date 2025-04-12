use super::CPU;
use crate::registers;
use registers::Flag::{Z, N, H, C};
use registers::{R8, R16};


impl CPU {
     /* ----- ALU ----- */

    // Each ALU op has an 8-bit and 16-bit version:

    // 8bit:
    // pub fn OP_a_u8(&mut self, value: u8) {
    // ...
    // }

    // 16bit:
    // pub fn OP_a_r16(&mut self, value: u16) {
    // ...
    // }

    // 8-bit ALU ops generally have three extraassociated functions
    // (INC and DEC are a bit different, but similar):

    // pub fn OP_a_r8(&mut self, r8: R8) {
    //     let value = self.reg.get(r8);
    //     self.OP_a_u8(value);
    // }

    // pub fn OP_a_at_hl(&mut self) {
    //     let hl = self.reg.get16(R16::HL);
    //     let value = self.mmu.readbyte(hl);
    //     self.OP_a_u8(value);
    // }

    // pub fn OP_a_n8(&mut self) {
    //     let n8 = self.fetch_byte();
    //     self.OP_a_u8(n8);
    // }

    // ADD
    pub fn add_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let sum = ra as u16 + value as u16;
        let result = sum as u8;

        self.reg.set(R8::A, result);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, (ra & 0xF) + (value & 0xF) > 0xF);
        self.reg.set_flag(C, sum > 0xFF);
    }

    pub fn add_hl_r16(&mut self, r16: R16) {
        let hl = self.reg.get16(R16::HL);
        let value = self.reg.get16(r16);

        let result = hl.wrapping_add(value);

        // Zero flag untouched
        self.reg.set_flag(N, false);
        self.reg
            .set_flag(H, (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        self.reg.set_flag(C, (hl as u32) + (value as u32) > 0xFFFF);

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
    pub fn adc_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);
        let carry = self.reg.get_flag(C) as u8;

        let sum = ra as u16 + carry as u16 + value as u16;
        let result = sum as u8;

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg
            .set_flag(H, (ra & 0xF) + (value & 0xF) + carry > 0xF);
        self.reg.set_flag(C, sum > 0xFF);

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
    pub fn sub_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(C, ra < value);

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
    pub fn sbc_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);
        let carry = self.reg.get_flag(C) as u8;

        let result = ra.wrapping_sub(value).wrapping_sub(carry);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (ra & 0xF) < ((value & 0xF) + carry));
        self.reg
            .set_flag(C, (ra as u16) < (value as u16 + carry as u16));

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
    pub fn and_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra & value;

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, true);
        self.reg.set_flag(C, false);

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
    pub fn or_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra | value;

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, false);

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
    pub fn xor_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra ^ value;

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, false);

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
    pub fn cp_a_u8(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(C, ra < value);
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
    pub fn inc_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, ((value & 0x0F) + 1) > 0x0F);
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
    pub fn dec_u8(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (value & 0x0F) == 0);
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