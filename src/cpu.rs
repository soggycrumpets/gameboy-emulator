use crate::memory::MMU;
use crate::registers;
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
        let byte = self.mmu.readbyte(self.reg.pc);
        self.reg.pc += 1;
        byte
    }

    pub fn fetchword(&mut self) -> u16 {
        let word = self.mmu.readword(self.reg.pc);
        self.reg.pc += 2;
        word
    }

    pub fn execute(&mut self) {
        let opcode = self.fetchbyte();

        match opcode {
            0x00 => {} // NOP
            0xC3 => {
                // JP n16
                let addr = self.fetchword();
                self.jp(addr);
            }
            _ => panic!("Unknown instruction: {:02x}", opcode),
        }
    }

    fn jp(&mut self, addr: u16) {
        self.reg.pc = addr;
    }

    /* ----- ALU ----- */
    fn add(&mut self, value: u8) {
        let ra = self.reg.a;

        let result = ra.wrapping_add(value);

        self.reg.a = result;

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = (ra & 0xF) + (value & 0xF) > 0xF;
        self.reg.f.carry = (ra as u16) + (value as u16) > 0xFF;
    }

    fn adc(&mut self, value: u8) {
        let ra = self.reg.a;
        let carry = self.reg.f.carry as u8;

        let result = ra.wrapping_add(carry).wrapping_add(value);

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = (ra & 0xF) + (value & 0xF) + carry > 0xF;
        self.reg.f.carry = (ra as u16) + (value as u16) + (carry as u16) > 0xFF;

        self.reg.a = result;
    }

    fn sub(&mut self, value: u8) {
        let ra = self.reg.a;

        let result = ra.wrapping_sub(value);

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = true;
        self.reg.f.half_carry = (ra & 0xF) < (value & 0xF);
        self.reg.f.carry = ra < value;

        self.reg.a = result;
    }

    fn sbc(&mut self, value: u8) {
        let ra = self.reg.a;
        let carry = self.reg.f.carry as u8;

        let result = ra.wrapping_sub(carry).wrapping_sub(value);

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = true;
        self.reg.f.half_carry = (ra & 0xF) < ((value & 0xF) + carry);
        self.reg.f.carry = (ra as u16) < (value as u16 + carry as u16);

        self.reg.a = result;
    }

    fn and(&mut self, value: u8) {
        let ra = self.reg.a;

        let result = ra & value;

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = true;
        self.reg.f.carry = false;

        self.reg.a = result;
    }

    fn or(&mut self, value: u8) {
        let ra = self.reg.a;

        let result = ra | value;

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = false;

        self.reg.a = result;
    }

    fn xor(&mut self, value: u8) {
        let ra = self.reg.a;

        let result = ra ^ value;

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = false;
        self.reg.f.carry = false;

        self.reg.a = result;
    }

    fn cp(&mut self, value: u8) {
        let ra = self.reg.a;

        let result = ra.wrapping_sub(value);

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = true;
        self.reg.f.half_carry = (ra & 0xF) < (value & 0xF);
        self.reg.f.carry = ra < value;
    }

    fn inc(&mut self) {
        let ra = self.reg.a;

        let result = ra.wrapping_add(1);

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = ((ra & 0x0F) + 1) > 0x0F;
        // Carry flag untouched

        self.reg.a = result;
    }

    fn dec(&mut self) {
        let ra = self.reg.a;

        let result = ra.wrapping_sub(1);

        self.reg.f.zero = result == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = ra == 0;
        // Carry flag untouched

        self.reg.a = result;
    }

    fn add_hl(&mut self, value: u16) {
        let hl = self.reg.get16(R16::HL);

        let result = hl.wrapping_add(value);

        // Zero flag untouched
        self.reg.f.subtract = false;
        self.reg.f.half_carry = (hl & 0x0FFF) + (value & 0x0FFF) > 0x0FFF;
        self.reg.f.carry = (hl as u32) + (value as u32) > 0xFFFF;

        self.reg.set16(R16::HL, result);
    }

    fn add_sp(&mut self, value: u8) {
        let sp = self.reg.get16(R16::SP);
        
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
