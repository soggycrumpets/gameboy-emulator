// pub const BOOTROM_START_ADDR: u16 = 0x0000;
// pub const TOP_OF_STACK_ADDR: u16 = 0xFFFE;

const ZERO_FLAG: u8 = 7;
const SUBTRACT_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

// 8-bit registers
#[derive(Clone, Copy, Debug)]
pub enum R8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

// 16-bit registers
#[derive(Clone, Copy, Debug)]
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

// F-register flags
pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
    sp: u16, // Stack pointer
    pc: u16, // Program counter
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
            sp: 0,
            pc: 0,
        }
    }

    pub fn get16(&self, register: R16) -> u16 {
        let (high_register, low_register) = match register {
            R16::AF => (R8::A, R8::F),
            R16::BC => (R8::B, R8::C),
            R16::DE => (R8::D, R8::E),
            R16::HL => (R8::H, R8::L),
            R16::SP => return self.sp,
            R16::PC => return self.pc,
        };

        let high = self.get(high_register);
        let low = self.get(low_register);

        let mut word: u16 = 0;
        word |= (high as u16) << 8;
        word |= low as u16;
        word
    }

    pub fn set16(&mut self, register: R16, word: u16) {
        let (high, low) = match register {
            R16::AF => (R8::A, R8::F),
            R16::BC => (R8::B, R8::C),
            R16::DE => (R8::D, R8::E),
            R16::HL => (R8::H, R8::L),
            R16::SP => {
                self.sp = word;
                return;
            }
            R16::PC => {
                self.pc = word;
                return;
            }
        };

        let high_value = (word >> 8) as u8;
        let low_value = word as u8;
        self.set(high, high_value);
        self.set(low, low_value);
    }

    pub fn set16_low(&mut self, register: R16, byte: u8) {
        let low_register = match register {
            R16::AF => R8::F,
            R16::BC => R8::C,
            R16::DE => R8::E,
            R16::HL => R8::L,
            R16::SP => {
                self.sp &= 0xFF00;
                self.sp |= byte as u16;
                return;
            }
            R16::PC => {
                self.pc &= 0xFF00;
                self.pc |= byte as u16;
                return;
            }
        };

        self.set(low_register, byte);
    }

    pub fn set16_high(&mut self, register: R16, byte: u8) {
        let high_register= match register {
            R16::AF => R8::A,
            R16::BC => R8::B,
            R16::DE => R8::D,
            R16::HL => R8::H,
            R16::SP => {
                self.sp &= 0x00FF;
                self.sp |= (byte as u16) << 8;
                return;
            }
            R16::PC => {
                self.pc &= 0x00FF;
                self.pc |= (byte as u16) << 8;
                return;
            }
        };

        self.set(high_register, byte);
    }

    pub fn get(&self, register: R8) -> u8 {
        match register {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::F => self.f.into(),
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    pub fn set(&mut self, register: R8, byte: u8) {
        match register {
            R8::A => self.a = byte,
            R8::B => self.b = byte,
            R8::C => self.c = byte,
            R8::D => self.d = byte,
            R8::E => self.e = byte,
            R8::F => self.f = (byte & 0xF0).into(), // Low bits always 0
            R8::H => self.h = byte,
            R8::L => self.l = byte,
        };
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.f.zero,
            Flag::N => self.f.subtract,
            Flag::H => self.f.half_carry,
            Flag::C => self.f.carry,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        match flag {
            Flag::Z => self.f.zero = value,
            Flag::N => self.f.subtract = value,
            Flag::H => self.f.half_carry = value,
            Flag::C => self.f.carry = value,
        }
    }
}

#[derive(Clone, Copy)]
struct Flags {
    zero: bool,
    subtract: bool,
    carry: bool,
    half_carry: bool,
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

mod debug {
    use super::*;
    impl Registers {
        pub fn print(&self) {
            print!("\nRegisters:\n");
            print!("A: {:02x}, ", self.a);
            print!("B: {:02x}, ", self.b);
            print!("C: {:02x}, ", self.c);
            print!("D: {:02x}, ", self.d);
            print!("E: {:02x}, ", self.e);
            print!("F: {:02x}, ", u8::from(self.f));
            print!("H: {:02x}, ", self.h);
            print!("L: {:02x}, ", self.l);
            print!("SP: {:04x}, ", self.sp);
            print!("PC {:04x}, ", self.pc);
            print!("\n\n");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_functions() {
        let mut registers = Registers::new();
        registers.set(R8::B, 4);
        registers.set(R8::C, 8);

        assert_eq!(registers.get(R8::B), 4);
        assert_eq!(registers.get(R8::C), 8);

        assert_eq!(registers.get16(R16::BC), 1032, "get_bc failed");

        let value = 1234;
        registers.set16(R16::BC, value);
        assert_eq!(registers.get16(R16::BC), value, "set_bc failed");
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
