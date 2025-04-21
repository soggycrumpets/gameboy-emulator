use super::*;
impl Cpu {
    pub fn push_r16(&mut self, r16: R16) {
        // print!(
        // "M: {}, T: {}",
        // self.instruction_m_cycles_remaining, self.instruction_t_cycles_remaining
        // );

        let lock = self.mmu.borrow().oam_lock;

        match self.instruction_m_cycles_remaining {
            // Instruction decoding
            4 => (),
            // Internal delay
            3 => (),
            // Write the high byte to memory
            2 => {
                let sp = self.reg.get16(R16::SP).wrapping_sub(1);
                self.reg.set16(R16::SP, sp);

                let high_byte = (self.reg.get16(r16) >> 8) as u8;
                self.write_byte(sp, high_byte);

                // println!("PUSH HI: {:02x} TO: {:04x} TICKS REMAINING: {} LOCKED: {}", high_byte, sp, self.instruction_t_cycles_remaining, lock);
            }
            // Write the low byte to memory
            1 => {
                // Decrement sp
                let sp = self.reg.get16(R16::SP).wrapping_sub(1);
                self.reg.set16(R16::SP, sp);

                let low_byte = self.reg.get16(r16) as u8;
                self.write_byte(sp, low_byte);

                // println!("PUSH LO: {:02x} TO: {:04x} TICKS REMAINING: {} LOCKED: {}", low_byte, sp, self.instruction_t_cycles_remaining, lock);
            }
            _ => unreachable!(),
        }
    }

    // todo!
    // This function will go away once all opcodes are m-cycle accurate
    pub fn push_r16_instant(&mut self, r16: R16) {
        // Decrement sp first
        let sp = self.reg.get16(R16::SP).wrapping_sub(2);
        self.reg.set16(R16::SP, sp);

        // Push the byte next
        let word = self.reg.get16(r16);
        self.write_word(sp, word);
    }

    // 2-byte POP
    pub fn pop_r16(&mut self, r16: R16) {
        let sp = self.reg.get16(R16::SP);

        // Pop the stack first
        let word = self.read_word(sp);
        self.reg.set16(r16, word);

        // Increment sp next
        if (sp as u32) + 2 > 0xFFFF {
            panic!("Stack underflow");
        }
        self.reg.set16(R16::SP, sp.wrapping_add(2));
    }

    pub fn ld_r8_r8(&mut self, r1: R8, r2: R8) {
        let byte = self.reg.get(r2);
        self.reg.set(r1, byte);
    }

    pub fn ld_r8_n8(&mut self, r8: R8) {
        let byte = self.fetch_byte();
        self.reg.set(r8, byte);
    }

    pub fn ld_r16_n16(&mut self, r16: R16) {
        let word = self.fetch_word();
        self.reg.set16(r16, word);
    }

    pub fn ld_at_hl_r8(&mut self, r8: R8) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            2 => (),
            // Write
            1 => {
                let byte = self.reg.get(r8);
                self.write_at_hl(byte);
            }
            _ => unreachable!(),
        }
    }

    pub fn ld_at_hl_n8(&mut self) {
        let byte = self.fetch_byte();
        self.write_at_hl(byte);
    }

    pub fn ld_r8_at_hl(&mut self, r8: R8) {
        match self.instruction_m_cycles_remaining {
            // Fetch
            2 => (),
            // Read
            1 => {
                let byte = self.read_at_hl();
                self.reg.set(r8, byte);
            }
            _ => unreachable!(),
        }
    }

    pub fn ld_at_r16_a(&mut self, r16: R16) {
        let addr = self.reg.get16(r16);
        let byte = self.reg.get(R8::A);
        self.write_byte(addr, byte);
    }

    pub fn ld_at_a16_a(&mut self) {
        let addr = self.fetch_word();
        let byte = self.reg.get(R8::A);
        self.write_byte(addr, byte);
    }

    pub fn ldh_at_a8_a(&mut self) {
        let a8 = self.fetch_byte();
        let ra = self.reg.get(R8::A);
        let addr = 0xFF00 + (a8 as u16);
        self.write_byte(addr, ra);
    }

    pub fn ldh_c_a(&mut self) {
        let ra = self.reg.get(R8::A);
        let rc = self.reg.get(R8::C);
        let addr = 0xFF00 + (rc as u16);
        self.write_byte(addr, ra);
    }

    pub fn ld_a_at_r16(&mut self, r16: R16) {
        let addr = self.reg.get16(r16);
        let byte = self.read_byte(addr);
        self.reg.set(R8::A, byte);
    }

    pub fn ld_a_at_a16(&mut self) {
        let a16 = self.fetch_word();
        let byte = self.read_byte(a16);
        self.reg.set(R8::A, byte);
    }

    pub fn ldh_a_a8(&mut self) {
        let a8 = self.fetch_byte();
        let addr = 0xFF00 + (a8 as u16);
        let byte = self.read_byte(addr);
        self.reg.set(R8::A, byte);
    }

    pub fn ldh_a_at_c(&mut self) {
        let rc = self.reg.get(R8::C);
        let addr = 0xFF00 + (rc as u16);
        let byte = self.read_byte(addr);
        self.reg.set(R8::A, byte);
    }

    // [HL+]/[HL-] use some repeated logic, so I extracted it out here
    fn ld_at_hl_a(&mut self) {
        let byte = self.reg.get(R8::A);
        self.write_at_hl(byte);
    }

    fn ld_a_at_hl(&mut self) {
        let byte = self.read_at_hl();
        self.reg.set(R8::A, byte);
    }

    fn step_hl_1(&mut self, increment: bool) {
        let step: i8 = if increment { 1 } else { -1 };

        let hl = self.reg.get16(R16::HL);
        let result = hl as i32 + step as i32;

        self.reg.set16(R16::HL, result as u16);
    }

    pub fn ld_at_hli_a(&mut self) {
        self.ld_at_hl_a();
        self.step_hl_1(true);
    }

    pub fn ld_at_hld_a(&mut self) {
        self.ld_at_hl_a();
        self.step_hl_1(false);
    }

    pub fn ld_a_at_hld(&mut self) {
        self.ld_a_at_hl();
        self.step_hl_1(false);
    }

    pub fn ld_a_at_hli(&mut self) {
        self.ld_a_at_hl();
        self.step_hl_1(true);
    }

    pub fn ld_at_n16_sp(&mut self) {
        let n16 = self.fetch_word();
        let sp = self.reg.get16(R16::SP);
        self.write_word(n16, sp);
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
