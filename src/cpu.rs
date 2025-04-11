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
            0xC3 => {
                // jp n16
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

    // LD Instructions
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

    fn ldh_c_a(&mut self) {
        let ra = self.reg.get(R8::A);
        self.mmu.writebyte(0xFF00 + 0xC, ra);
    }

    fn ld_a_r16(&mut self, r16: R16) {
        let addr = self.reg.get16(r16);
        let value = self.mmu.readbyte(addr);
        self.reg.set(R8::A, value);
    }

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

    fn inc(&mut self) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_add(1);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, ((ra & 0x0F) + 1) > 0x0F);
        // Carry flag untouched

        self.reg.set(R8::A, result);
    }

    fn dec(&mut self) {
        let ra = self.reg.get(R8::A);

        let result = ra.wrapping_sub(1);

        self.reg.set_flag(Z, result == 0);
        self.reg.set_flag(N, false);
        self.reg.set_flag(H, ra == 0);
        // Carry flag untouched

        self.reg.set(R8::A, result);
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

    // fn add_sp(&mut self, value: u8) {
    // let sp = self.reg.get16(SP);
    // }
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
