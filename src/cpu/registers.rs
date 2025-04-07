const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
}
impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0.into(),
            h: 0,
            l: 0,
        }
    }

    pub fn get_combined(&self, high: Register, low: Register) -> u16 {
        let high_value = self.get_register(high);
        let low_value = self.get_register(low);

        let mut combined: u16 = 0;
        combined |= (high_value as u16) << 8;
        combined |= self.c as u16;
        combined
    }

    pub fn set_combined(&mut self, bits: u16, high: Register, low: Register) {
        let high_value = (bits >> 8) as u8;
        let low_value = bits as u8;
        self.set_register(high, high_value);
        self.set_register(low, low_value);
    }

    fn get_register(&self, register: Register) -> u8 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::E => self.e,
            Register::H => self.h,
            Register::L => self.l,
        }
    }

    fn set_register(&mut self, register: Register, value: u8) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
            Register::C => self.c = value,
            Register::D => self.d = value,
            Register::E => self.e = value,
            Register::H => self.h = value,
            Register::L => self.l = value,
        };
    }
}

#[derive(Clone, Copy)]
pub struct Flags {
    pub zero: bool,
    pub subtract: bool,
    pub carry: bool,
    pub half_carry: bool,
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            zero: value & (1 << ZERO_FLAG) != 0,
            subtract: value & (1 << SUBTRACT_FLAG) != 0,
            carry: value & (1 << CARRY_FLAG) != 0,
            half_carry: value & (1 << HALF_CARRY_FLAG) != 0,
        }
    }
}

impl From<Flags> for u8 {
    fn from(register: Flags) -> Self {
        let mut value = 0;
        value |= (register.zero as u8) << ZERO_FLAG;
        value |= (register.subtract as u8) << SUBTRACT_FLAG;
        value |= (register.carry as u8) << CARRY_FLAG;
        value |= (register.half_carry as u8) << HALF_CARRY_FLAG;
        value
    }
}

pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_functions() {
        let mut registers = Registers::new();
        registers.b = 4;
        registers.c = 8;

        assert_eq!(
            registers.get_combined(Register::B, Register::C),
            1032,
            "get_bc failed"
        );

        let value = 1234;
        registers.set_combined(value, Register::B, Register::C);
        assert_eq!(
            registers.get_combined(Register::B, Register::C),
            value,
            "set_bc failed"
        );
    }

    #[test]
    fn test_flag_functions() {
        let mut flags = Flags::from(0);
        flags.carry = true;
        assert_eq!(u8::from(flags), 16);
        flags.half_carry = true;
        assert_eq!(u8::from(flags), 48);
        flags.subtract = true;
        assert_eq!(u8::from(flags), 112);
        flags.zero = true;
        assert_eq!(u8::from(flags), 240);
    }
}
