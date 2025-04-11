const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

use crate::constants::PROGRAM_START_ADDR;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: Flags,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
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
            sp: 0xFFFF, // Starts at the top of the stack
            pc: PROGRAM_START_ADDR, // Program counter always starts here when the device is powered on
        }
    }

    pub fn get16(&self, register: R16) -> u16 {
        let (high_register, low_register) = match register {
            R16::BC => (R8::B, R8::C),
            R16::DE => (R8::D, R8::E),
            R16::HL => (R8::H, R8::L),
            R16::SP => return self.sp,
        };

        let high= self.get(high_register);
        let low= self.get(low_register);

        let mut combined: u16 = 0;
        combined |= (high as u16) << 8;
        combined |= low as u16;
        combined
    }

    pub fn set16(&mut self, bits: u16, register: R16) {
        let (high, low) = match register {
            R16::BC => (R8::B, R8::C),
            R16::DE => (R8::D, R8::E),
            R16::HL => (R8::H, R8::L),
            R16::SP => {
                self.sp = bits;
                return;
            }
        };

        let high_value = (bits >> 8) as u8;
        let low_value = bits as u8;
        self.set(high, high_value);
        self.set(low, low_value);
    }

    pub fn get(&self, register: R8) -> u8 {
        match register {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    pub fn set(&mut self, register: R8, value: u8) {
        match register {
            R8::A => self.a = value,
            R8::B => self.b = value,
            R8::C => self.c = value,
            R8::D => self.d = value,
            R8::E => self.e = value,
            R8::H => self.h = value,
            R8::L => self.l = value,
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

#[derive(Clone, Copy)]
pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Clone, Copy)]
pub enum R16 {
    BC,
    DE,
    HL,
    SP,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_functions() {
        let mut R8 = Registers::new();
        R8.b = 4;
        R8.c = 8;

        assert_eq!(R8.get(R8::B), 4);
        assert_eq!(R8.get(R8::C), 8);

        assert_eq!(R8.get16(R16::BC), 1032, "get_bc failed");

        let value = 1234;
        R8.set16(value, R16::BC);
        assert_eq!(R8.get16(R16::BC), value, "set_bc failed");
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
