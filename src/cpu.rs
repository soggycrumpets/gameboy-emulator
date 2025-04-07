mod registers;
use registers::Registers;
use registers::Register as ArithmeticTarget;

pub struct CPU {
    registers: Registers,
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => self.registers.a = self.add(target),
        }
    }

    fn add(&mut self, target: ArithmeticTarget) -> u8 {
        let value = self.get_register_value(target);
        let (new_value, overflowed) = self.registers.a.overflowing_add(value);

        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = overflowed;
        self.registers.f.carry = (value & 0xf) + (self.registers.a & 0xf) > 0xf;

        new_value
    }

    // fn addhl(&mut self, target: ArithmeticTarget) -> u16 {

    // }

    fn get_register_value(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::A => self.registers.a,
            ArithmeticTarget::B => self.registers.b,
            ArithmeticTarget::C => self.registers.c,
            ArithmeticTarget::D => self.registers.d,
            ArithmeticTarget::E => self.registers.e,
            ArithmeticTarget::H => self.registers.h,
            ArithmeticTarget::L => self.registers.l,
        }
    }
}

enum Instruction {
    ADD(ArithmeticTarget),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.registers.a = 100;
        cpu.registers.c = 75;
        let instruction = Instruction::ADD(ArithmeticTarget::C);
        cpu.execute(instruction);

        assert_eq!(cpu.registers.a, 175);
        assert!(!cpu.registers.f.carry);
        assert!(!cpu.registers.f.half_carry);
        assert!(!cpu.registers.f.subtract);
        assert!(!cpu.registers.f.zero);
    }
}
