use std::ptr::addr_of;

use crate::memory::MMU;
use crate::registers;
use registers::Flag::*;
use registers::Registers;
use registers::{R8, R16};

pub struct CPU {
    reg: Registers,
    pub mmu: MMU,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            reg: Registers::new(),
            mmu: MMU::new(),
        }
    }

    pub fn fetchbyte(&mut self) -> u8 {
        let pc = self.reg.get16(R16::PC);
        let byte = self.mmu.readbyte(pc);

        let next_addr = pc + 1;
        self.reg.set16(R16::PC, next_addr);
        byte
    }

    pub fn fetchword(&mut self) -> u16 {
        let pc = self.reg.get16(R16::PC);
        let word = self.mmu.readword(pc);

        let next_addr = pc + 2;
        self.reg.set16(R16::PC, next_addr);
        word
    }

    pub fn execute(&mut self) {
        let opcode = self.fetchbyte();
        println!("{:02x}", opcode);

        match opcode {
            0x00 => {} // NOP
            0x01 => {
                // LD BC, n16
                let n16 = self.fetchword();
                self.ld_r16_n16(R16::BC, n16);
            }
            0x02 => self.ld_r16_a(R16::BC), // LD BC, A
            0x03 => todo!("INC BC"), // INC BC
            0x04 => self.inc(R8::B), // INC B
            0x05 => self.dec(R8::B), // DEC B
            0x06 => {
                // LD B, n8
                let n8 = self.fetchbyte();
                self.ld_r8_n8(R8::B, n8);
            }
            0x07 => todo!("RLCA"),// RLCA
            0x08 => {
                // LD [n16], SP
                let n16 = self.fetchword();
                self.ld_n16_sp(n16);
            }
            0x09 => {
                // ADD HL, BC
                let bc = self.reg.get16(R16::BC);
                self.add_hl(bc);
            }
            0x0A => self.ld_a_r16(R16::BC), // LD A, R16
            0x0B => todo!("DEC BC"),
            0x0C => self.inc(R8::C), // INC C
            0x0D => self.dec(R8::C), // DEC C
            0x0E =>  {
                // LD C, n8
                let n8 = self.fetchbyte();
                self.ld_r8_n8(R8::C, n8);
            }
            0x0F => todo!("RRCS"),// RRCS
            0x10 => todo!("STOP n8"), // STOP n8
            0x11 => {
                // LD DE, n16
                let n16 = self.fetchword();
                self.ld_r16_n16(R16::DE, n16);
            }
            0x12 => self.ld_r16_a(R16::DE), // LD DE, A
            0x13 => todo!("INC DE"), // INC DE
            0x14 => self.inc(R8::D), // INC D
            0x15 => self.dec(R8::D),
            0x16 => {
                // LD D, n8
                let n8 = self.fetchbyte();
                self.ld_r8_n8(R8::D, n8);
            }
            0x17 => todo!("RLA"), // RLA
            0x18 => todo!("JR e8"), // JR e8
            0x19 => todo!("ADD HL, DE"), // ADD HL, DE
            0x1A => self.ld_a_r16(R16::DE), // LD A, [DE]
            0x1B => todo!("DEC DE"), // DEC DE
            0x1C => self.inc(R8::E), // INC E
            0x01D => self.dec(R8::E), // DEC E
            0x21 => {
                // LD HL, n16
                let n16 = self.fetchword();
                self.ld_r16_n16(R16::HL, n16);
            }
            0x31 => {
                // LD SP, n16
                let n16 = self.fetchword();
                self.ld_r16_n16(R16::SP, n16);
            }
            0xC3 => {
                // JP n16
                let n16 = self.fetchword();
                self.jp_n16(n16)
            }
            0xC1 => self.pop(R16::BC),  // pop BC
            0xD1 => self.pop(R16::DE),  // pop DE
            0xE1 => self.pop(R16::HL),  // pop HL
            0xF1 => self.pop(R16::AF),  // pop AF
            0xC5 => self.push(R16::BC), // push BC
            0xD5 => self.push(R16::DE), // push DE
            0xE5 => self.push(R16::HL), // push HL
            0xF5 => self.push(R16::AF), // push AF
            _ => panic!("Unknown instruction: {:02x}", opcode),
        }
    }

    // LD instructions
    fn ld_r8_r8(&mut self, r1: R8, r2: R8) {
        let r2_value = self.reg.get(r2);
        self.reg.set(r1, r2_value);
    }

    fn ld_r8_n8(&mut self, r8: R8, n8: u8) {
        self.reg.set(r8, n8);
    }

    fn ld_r16_n16(&mut self, r16: R16, n16: u16) {
        self.reg.set16(r16, n16);
    }

    fn ld_hl_r8(&mut self, r8: R8) {
        let addr = self.reg.get16(R16::HL);
        let value = self.reg.get(r8);
        self.mmu.writebyte(addr, value);
    }

    fn ld_hl_n8(&mut self, n8: u8) {
        let addr = self.reg.get16(R16::HL);
        self.mmu.writebyte(addr, n8);
    }

    fn ld_r8_hl(&mut self, r8: R8) {
        let addr = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(addr);
        self.reg.set(r8, value);
    }

    fn ld_r16_a(&mut self, r16: R16) {
        let addr = self.reg.get16(r16);
        let ra = self.reg.get(R8::A);
        self.mmu.writebyte(addr, ra);
    }

    fn ld_n16_a(&mut self, n16: u16) {
        let ra = self.reg.get(R8::A);
        self.mmu.writebyte(n16, ra);
    }

    fn ldh_n8_a(&mut self, n8: u8) {
        let ra = self.reg.get(R8::A);
        let addr = 0xFF00 + (n8 as u16);
        self.mmu.writebyte(addr, ra);
    }

    fn ldh_c_a(&mut self) {
        let ra = self.reg.get(R8::A);
        let rc = self.reg.get(R8::C);
        let addr = 0xFF00 + (rc as u16);
        self.mmu.writebyte(addr, ra);
    }

    fn ld_a_r16(&mut self, r16: R16) {
        let addr = self.reg.get16(r16);
        let value = self.mmu.readbyte(addr);
        self.reg.set(R8::A, value);
    }

    fn ld_a_n16(&mut self, n16: u16) {
        let value = self.mmu.readbyte(n16);
        self.reg.set(R8::A, value);
    }

    fn ldh_a_n8(&mut self, n8: u8) {
        let addr = 0xFF00 + (n8 as u16);
        let value = self.mmu.readbyte(addr);
        self.reg.set(R8::A, value);
    }

    fn ldh_a_c(&mut self) {
        let rc = self.reg.get(R8::C);
        let addr = 0xFF00 + (rc as u16);
        let value = self.mmu.readbyte(addr);
        self.reg.set(R8::A, value);
    }

    fn ld_hli_a(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let a = self.reg.get(R8::A);

        self.mmu.writebyte(hl, a);

        self.reg.set16(R16::HL, hl + 1);
    }

    fn ld_hld_a(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let a = self.reg.get(R8::A);

        self.mmu.writebyte(hl, a);

        self.reg.set16(R16::HL, hl - 1);
    }

    fn ld_a_hld(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);

        self.reg.set(R8::A, value);

        self.reg.set16(R16::HL, hl - 1);
    }

    fn ld_a_hli(&mut self) {
        let hl = self.reg.get16(R16::HL);
        let value = self.mmu.readbyte(hl);

        self.reg.set(R8::A, value);

        self.reg.set16(R16::HL, hl + 1);
    }

    fn ld_sp_n16(&mut self, n16: u16) {
        self.reg.set16(R16::SP, n16);
    }

    fn ld_n16_sp(&mut self, n16: u16) {
        let sp = self.reg.get16(R16::SP);
        self.mmu.writeword(n16, sp);
    }

    fn ld_hl_sp_e8(&mut self, n8: u8) {
        let sp = self.reg.get16(R16::SP);
        let e8 = (n8 as i8) as i16;

        let result = (sp as i16).wrapping_add(e8) as u16;

        let sp_low = sp as u8;

        self.reg.set_flag(Z, false);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, (sp_low & 0x0F) + (n8 & 0x0F) > 0x0F);
        self.reg.set_flag(
            C,
            ((sp_low as u16) & 0x00FF) + ((n8 as u16) & 0x00FF) > 0xFF,
        );

        self.reg.set16(R16::HL, result);
    }

    fn ld_sp_hl(&mut self) {
        let hl = self.reg.get16(R16::HL);
        self.reg.set16(R16::SP, hl);
    }

    // Stack instructions
    fn push(&mut self, r16: R16) {
        // Ensure push target is valid
        match r16 {
            R16::AF | R16::BC | R16::DE | R16::HL => {}
            _ => panic!("Invalid pop target {:?}", r16),
        }

        // Decrement sp
        let sp = self.reg.get16(R16::SP).wrapping_sub(2);
        self.reg.set16(R16::SP, sp);

        // Push the value in the register to the stack
        let value = self.reg.get16(r16);
        self.mmu.writeword(sp, value);
    }

    fn pop(&mut self, r16: R16) {
        // Ensure pop target is valid
        match r16 {
            R16::AF | R16::BC | R16::DE | R16::HL => {}
            _ => panic!("Invalid pop target {:?}", r16),
        }

        let sp = self.reg.get16(R16::SP);

        // Pop the stack into the register
        let value = self.mmu.readword(sp);
        self.reg.set16(r16, value);

        // Increment sp
        self.reg.set16(R16::SP, sp + 2);
    }

    fn jp_n16(&mut self, addr: u16) {
        self.reg.set16(R16::PC, addr);
    }

    /* ----- 8-bit ALU ----- */
    fn add(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_add(value);

        self.reg.set(R8::A, result);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, (ra & 0xF) + (value & 0xF) > 0xF);
        self.reg.set_flag(C, (ra as u16) + (value as u16) > 0xFF);
    }

    fn adc(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);
        let carry = self.reg.get_flag(C) as u8;

        let result = ra.wrapping_add(carry).wrapping_add(value);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg
            .set_flag(H, (ra & 0xF) + (value & 0xF) + carry > 0xF);
        self.reg
            .set_flag(C, (ra as u16) + (value as u16) + (carry as u16) > 0xFF);

        self.reg.set(R8::A, result);
    }

    fn sub(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(C, ra < value);

        self.reg.set(R8::A, result);
    }

    fn sbc(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);
        let carry = self.reg.get_flag(C) as u8;

        let result = ra.wrapping_sub(carry).wrapping_sub(value);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (ra & 0xF) < ((value & 0xF) + carry));
        self.reg
            .set_flag(C, (ra as u16) < (value as u16 + carry as u16));

        self.reg.set(R8::A, result);
    }

    fn and(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra & value;

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, true);
        self.reg.set_flag(C, false);

        self.reg.set(R8::A, result);
    }

    fn or(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra | value;

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, false);

        self.reg.set(R8::A, result);
    }

    fn xor(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra ^ value;

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, false);
        self.reg.set_flag(C, false);

        self.reg.set(R8::A, result);
    }

    fn cp(&mut self, value: u8) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(value);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, true);
        self.reg.set_flag(H, (ra & 0xF) < (value & 0xF));
        self.reg.set_flag(C, ra < value);
    }

    fn inc(&mut self, r8: R8) {
        let value = self.reg.get(r8);

        let result = value.wrapping_add(1);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, ((value & 0x0F) + 1) > 0x0F);
        // Carry flag untouched

        self.reg.set(r8, result);
    }

    fn dec(&mut self, r8: R8) {
        let value = self.reg.get(R8::A);

        let result = value.wrapping_sub(1);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, value == 0);
        // Carry flag untouched

        self.reg.set(r8, result);
    }

    /* ----- 16-bit ALU ----- */
    fn add_hl(&mut self, value: u16) {
        let hl = self.reg.get16(R16::HL);

        let result = hl.wrapping_add(value);

        // Zero flag untouched
        self.reg.set_flag(N, false);
        self.reg
            .set_flag(H, (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF);
        self.reg.set_flag(C, (hl as u32) + (value as u32) > 0xFFFF);

        self.reg.set16(R16::HL, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        // let mut cpu = CPU::new();
        // cpu.reg.a = 100;
        // cpu.reg.c = 75;
        // let instruction = Instruction::ADD(R8::C);
        // cpu.execute(instruction);

        // assert_eq!(cpu.reg.a, 175);
        // assert!(!cpu.reg.f.carry);
        // assert!(!cpu.reg.f.half_carry);
        // assert!(!cpu.reg.f.subtract);
        // assert!(!cpu.reg.f.zero);
    }
}
