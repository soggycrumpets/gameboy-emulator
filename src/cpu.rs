use crate::registers;
use crate::memory::RAM;
use registers::Register;
use registers::Register16;
use registers::Registers;

pub struct CPU {
    registers: Registers,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => self.registers.a = self.add(target),
            Instruction::ADC(target) => self.registers.a = self.add_carry(target),
            _ => panic!("Instruction not yet implemented"),
        }
    }

    fn add(&mut self, target: Register) -> u8 {
        let value = self.registers.get(target);
        let (new_value, overflowed) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = overflowed;
        self.registers.f.carry = (value & 0xf) + (self.registers.a & 0xf) > 0xf;

        new_value
    }

    fn add_carry(&mut self, target: Register) -> u8 {
        // First, add the carry flag
        let (new_value1, overflowed1) = self.registers.a.overflowing_add(self.registers.f.carry as u8);
        
        // Then add the value
        let value = self.registers.get(target);
        let (new_value2, overflowed2) = new_value1.overflowing_add(value);

        // Overflow could have occurred in either operation
        let overflowed = overflowed1 || overflowed2;

        self.registers.f.zero = new_value2 == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = overflowed;
        self.registers.f.carry = (value & 0xf) + (self.registers.a & 0xf) > 0xf;

        new_value2 
    }

    // fn addhl(&mut self, target: Register16) -> u16 {
        // self.registers.get_16(high, low)

    // }
    
    // fn sub(&mut self, target: Register) -> u8 {

    // }
}

enum Instruction {
    ADD(Register),
    ADDHL(Register16),
    ADC(Register),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.registers.a = 100;
        cpu.registers.c = 75;
        let instruction = Instruction::ADD(Register::C);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 175);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.subtract);
        assert!(!cpu.registers.f.zero);
    }
}
