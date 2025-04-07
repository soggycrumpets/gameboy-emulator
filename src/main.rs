const CARRY_FLAG: u8 = 4;
const HALF_CARRY_FLAG: u8 = 5;
const SUBTRACT_FLAG: u8 = 6;
const ZERO_FLAG: u8 = 7;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
}
impl Registers {
    fn new() -> Self {
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

    fn get_bc(&self) -> u16 {
        let mut bc: u16 = 0;
        bc |= (self.b as u16) << 8;
        bc |= self.c as u16;
        bc
    }

    fn set_bc(&mut self, bits: u16) {
        self.b = (bits >> 8) as u8;
        self.c = bits as u8;
    }
}

#[derive(Clone, Copy)]
struct Flags {
    carry: bool,
    half_carry: bool,
    subtract: bool,
    zero: bool,
}
impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            carry: value & (1 << CARRY_FLAG) != 0,
            half_carry: value & (1 << HALF_CARRY_FLAG) != 0,
            subtract: value & (1 << SUBTRACT_FLAG) != 0,
            zero: value & (1 << ZERO_FLAG) != 0,
        }
    }
}
impl From<Flags> for u8 {
    fn from(register: Flags) -> Self {
        let mut value = 0;
        value |= (register.carry as u8) << CARRY_FLAG;
        value |= (register.half_carry as u8) << HALF_CARRY_FLAG;
        value |= (register.subtract as u8) << SUBTRACT_FLAG;
        value |= (register.zero as u8) << ZERO_FLAG;
        value
    }
}

fn main() {}

#[test]
fn test_register_functions() {
    let mut registers = Registers::new();
    registers.b = 4;
    registers.c = 8;

    assert_eq!(registers.get_bc(), 1032, "get_bc failed");

    let value = 1234;
    registers.set_bc(value);
    assert_eq!(registers.get_bc(), value, "set_bc failed");
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