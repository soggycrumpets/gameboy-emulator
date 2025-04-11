use crate::memory::MMU;
use crate::registers;
use registers::R8;
use registers::R16;
use registers::Registers;

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
        let byte = self.mmu.memory[self.reg.pc as usize];
        self.reg.pc += 1;
        byte
    }

    pub fn fetchword(&mut self, ram: &mut MMU) -> u16 {
        let lowbyte = ram.memory[(self.reg.pc + 1) as usize];
        let highbyte = ram.memory[self.reg.pc as usize];
        let word = ((lowbyte as u16) << 8) | highbyte as u16;
        self.reg.pc += 2;
        word
    }

    pub fn execute(&mut self) {
        let opcode = self.fetchbyte();

        match opcode {
            0x00 => {} // NOP
            0xC3 => {} // JP 16
            // _ => print!("{:02x}", opcode),
            _ => panic!("Unknown instruction: {:02x}", opcode),
        }
    }

    fn ld_r8_r8(reg1: R8, reg2: R8) {}

    fn jp_n16(&mut self, ram: &mut MMU) {
        let word = self.fetchword(ram);
    }

    // ALU
    fn add(&mut self, target: R8) -> u8 {
        let value = self.reg.get(target);
        let (new_value, overflowed) = self.reg.a.overflowing_add(value);

        self.reg.f.zero = new_value == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = overflowed;
        self.reg.f.carry = (value & 0xf) + (self.reg.a & 0xf) > 0xf;

        new_value
    }

    fn adc(&mut self, target: R8) -> u8 {
        // First, add the carry flag
        let (new_value1, overflowed1) = self.reg.a.overflowing_add(self.reg.f.carry as u8);

        // Then add the value
        let value = self.reg.get(target);
        let (new_value2, overflowed2) = new_value1.overflowing_add(value);

        // Overflow could have occurred in either operation
        let overflowed = overflowed1 | overflowed2;

        self.reg.f.zero = new_value2 == 0;
        self.reg.f.subtract = false;
        self.reg.f.half_carry = overflowed;
        self.reg.f.carry = (value & 0xf) + (self.reg.a & 0xf) > 0xf;

        new_value2
    }

    // fn addhl(&mut self, target: R16) -> u16 {
    // self.reg.get_16(high, low)

    // }

    // fn sub(&mut self, target: R8) -> u8 {

    // }
}

enum Instruction {
    ADD(R8),
    ADDHL(R16),
    ADC(R8),
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
