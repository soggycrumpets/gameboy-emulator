use super::*;
impl Cpu {
    // 2-byte PUSH
    pub fn push_r16(&mut self, r16: R16) {
        // Decrement sp
        let sp = self.reg.get16(R16::SP).wrapping_sub(2);
        self.reg.set16(R16::SP, sp);

        // Push the value in the register to the stack
        let value = self.reg.get16(r16);
        self.mmu.writeword(sp, value);
    }

    // 2-byte POP
    pub fn pop_r16(&mut self, r16: R16) {
        let sp = self.reg.get16(R16::SP);

        // Pop the stack into the register
        let value = self.mmu.readword(sp);
        self.reg.set16(r16, value);

        // Increment sp
        self.reg.set16(R16::SP, sp + 2);
    }

    pub fn ld_r8_r8(&mut self, r1: R8, r2: R8) {
        let r2_value = self.reg.get(r2);
        self.reg.set(r1, r2_value);
    }

    pub fn ld_r8_n8(&mut self, r8: R8) {
        let n8 = self.fetch_byte();
        self.reg.set(r8, n8);
    }

    pub fn ld_r16_n16(&mut self, r16: R16) {
        let n16 = self.fetch_word();
        self.reg.set16(r16, n16);
    }

    pub fn ld_at_hl_r8(&mut self, r8: R8) {
        let addr = self.reg.get16(R16::HL);
        let value = self.reg.get(r8);
        self.mmu.writebyte(addr, value);
    }

    pub fn ld_hl_n8(&mut self) {
        let n8 = self.fetch_byte();
        let addr = self.reg.get16(R16::HL);
        self.mmu.writebyte(addr, n8);
    }

    pub fn ld_r8_at_hl(&mut self, r8: R8) {
        let addr = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(addr);
        self.reg.set(r8, value);
    }

    pub fn ld_r16_a(&mut self, r16: R16) {
        let addr = self.reg.get16(r16);
        let ra = self.reg.get(R8::A);
        self.mmu.writebyte(addr, ra);
    }

    pub fn ld_at_a16_a(&mut self) {
        let a16 = self.fetch_word();
        let ra = self.reg.get(R8::A);
        self.mmu.writebyte(a16, ra);
    }

    pub fn ldh_at_a8_a(&mut self) {
        let a8 = self.fetch_byte();
        let ra = self.reg.get(R8::A);
        let addr = 0xFF00 + (a8 as u16);
        self.mmu.writebyte(addr, ra);
    }

    pub fn ldh_c_a(&mut self) {
        let ra = self.reg.get(R8::A);
        let rc = self.reg.get(R8::C);
        let addr = 0xFF00 + (rc as u16);
        self.mmu.writebyte(addr, ra);
    }

    pub fn ld_a_at_r16(&mut self, r16: R16) {
        let addr = self.reg.get16(r16);
        let value = self.mmu.readbyte(addr);
        self.reg.set(R8::A, value);
    }

    pub fn ld_a_at_a16(&mut self) {
        let a16 = self.fetch_word();
        let value = self.mmu.readbyte(a16);
        self.reg.set(R8::A, value);
    }

    pub fn ldh_a_a8(&mut self) {
        let a8 = self.fetch_byte();
        let addr = 0xFF00 + (a8 as u16);
        let value = self.mmu.readbyte(addr);
        self.reg.set(R8::A, value);
    }

    pub fn ldh_a_at_c(&mut self) {
        let rc = self.reg.get(R8::C);
        let addr = 0xFF00 + (rc as u16);
        let value = self.mmu.readbyte(addr);
        self.reg.set(R8::A, value);
    }

    pub fn ld_at_hli_a(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let a = self.reg.get(R8::A);

        self.mmu.writebyte(hl, a);

        self.reg.set16(R16::HL, hl + 1);
    }

    pub fn ld_at_hld_a(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let a = self.reg.get(R8::A);

        self.mmu.writebyte(hl, a);

        self.reg.set16(R16::HL, hl - 1);
    }

    pub fn ld_a_at_hld(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);

        self.reg.set(R8::A, value);

        self.reg.set16(R16::HL, hl - 1);
    }

    pub fn ld_a_at_hli(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);

        self.reg.set(R8::A, value);

        self.reg.set16(R16::HL, hl + 1);
    }

    pub fn ld_at_n16_sp(&mut self) {
        let n16 = self.fetch_word();
        let sp = self.reg.get16(R16::SP);
        self.mmu.writeword(n16, sp);
    }

    // This is a weird one. I'm having it use a function from alu.rs ADD 16-bit
    pub fn ld_hl_sp_e8(&mut self) {
        let result = self.calc_sp_plus_e8();
        self.reg.set16(R16::HL, result);
    }

    pub fn ld_sp_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        self.reg.set16(R16::SP, hl);
    }
}
