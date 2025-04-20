use lazy_static::lazy_static;
struct Operand {
    pub name: &'static str,
    pub immediate: bool,
    pub bytes: u8,
}
struct Flag {
    pub z: &'static str,
    pub n: &'static str,
    pub h: &'static str,
    pub c: &'static str,
}
struct Opcode {
    name: &'static str,
    mnemonic: &'static str,
    bytes: u8,
    cycles: Vec<u8>,
    operands: Vec<Operand>,
    immediate: bool,
    flags: Flag,
}
lazy_static! {
    static ref UNPREFIXED_OPCODES: [Opcode; 256usize] = [
        Opcode {
            name: "0x00",
            mnemonic: "NOP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x01",
            mnemonic: "LD",
            bytes: 3u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "BC",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x02",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "BC",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x03",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "BC",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x04",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x05",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x06",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x07",
            mnemonic: "RLCA",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "0",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x08",
            mnemonic: "LD",
            bytes: 3u8,
            cycles: vec!(20u8),
            operands: vec!(
                Operand {
                    name: "a16",
                    immediate: false,
                    bytes: 2u8,
                },
                Operand {
                    name: "SP",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x09",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "BC",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x0A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "BC",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x0B",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "BC",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x0C",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x0D",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x0E",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x0F",
            mnemonic: "RRCA",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "0",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x10",
            mnemonic: "STOP",
            bytes: 2u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "n8",
                immediate: true,
                bytes: 1u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x11",
            mnemonic: "LD",
            bytes: 3u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "DE",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x12",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "DE",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x13",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "DE",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x14",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x15",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x16",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x17",
            mnemonic: "RLA",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "0",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x18",
            mnemonic: "JR",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(Operand {
                name: "e8",
                immediate: true,
                bytes: 1u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x19",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "DE",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x1A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "DE",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x1B",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "DE",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x1C",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x1D",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x1E",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x1F",
            mnemonic: "RRA",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "0",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x20",
            mnemonic: "JR",
            bytes: 2u8,
            cycles: vec!(12u8, 8u8),
            operands: vec!(
                Operand {
                    name: "NZ",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "e8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x21",
            mnemonic: "LD",
            bytes: 3u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x22",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x23",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x24",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x25",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x26",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x27",
            mnemonic: "DAA",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "-",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x28",
            mnemonic: "JR",
            bytes: 2u8,
            cycles: vec!(12u8, 8u8),
            operands: vec!(
                Operand {
                    name: "Z",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "e8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x29",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x2A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x2B",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x2C",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x2D",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x2E",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x2F",
            mnemonic: "CPL",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "1",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x30",
            mnemonic: "JR",
            bytes: 2u8,
            cycles: vec!(12u8, 8u8),
            operands: vec!(
                Operand {
                    name: "NC",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "e8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x31",
            mnemonic: "LD",
            bytes: 3u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "SP",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x32",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x33",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "SP",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x34",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(12u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x35",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(12u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x36",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x37",
            mnemonic: "SCF",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "0",
                h: "0",
                c: "1",
            }
        },
        Opcode {
            name: "0x38",
            mnemonic: "JR",
            bytes: 2u8,
            cycles: vec!(12u8, 8u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "e8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x39",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "SP",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x3A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x3B",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "SP",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x3C",
            mnemonic: "INC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x3D",
            mnemonic: "DEC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0x3E",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x3F",
            mnemonic: "CCF",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x40",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x41",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x42",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x43",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x44",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x45",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x46",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x47",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x48",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x49",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x4A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x4B",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x4C",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x4D",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x4E",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x4F",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x50",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x51",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x52",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x53",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x54",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x55",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x56",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x57",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x58",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x59",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x5A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x5B",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x5C",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x5D",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x5E",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x5F",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x60",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x61",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x62",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x63",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x64",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x65",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x66",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x67",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x68",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x69",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x6A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x6B",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x6C",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x6D",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x6E",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x6F",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x70",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x71",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x72",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x73",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x74",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x75",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x76",
            mnemonic: "HALT",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x77",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x78",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x79",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x7A",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x7B",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x7C",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x7D",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x7E",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x7F",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x80",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x81",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x82",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x83",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x84",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x85",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x86",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x87",
            mnemonic: "ADD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x88",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x89",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x8A",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x8B",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x8C",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x8D",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x8E",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x8F",
            mnemonic: "ADC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x90",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x91",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x92",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x93",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x94",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x95",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x96",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x97",
            mnemonic: "SUB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "1",
                n: "1",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x98",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x99",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x9A",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x9B",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x9C",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x9D",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x9E",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0x9F",
            mnemonic: "SBC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "-",
            }
        },
        Opcode {
            name: "0xA0",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA1",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA2",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA3",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA4",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA5",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA6",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA7",
            mnemonic: "AND",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xA8",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xA9",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xAA",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xAB",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xAC",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xAD",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xAE",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xAF",
            mnemonic: "XOR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "1",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB0",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB1",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB2",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB3",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB4",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB5",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB6",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB7",
            mnemonic: "OR",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xB8",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xB9",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xBA",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xBB",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xBC",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xBD",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xBE",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xBF",
            mnemonic: "CP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "1",
                n: "1",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xC0",
            mnemonic: "RET",
            bytes: 1u8,
            cycles: vec!(20u8, 8u8),
            operands: vec!(Operand {
                name: "NZ",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC1",
            mnemonic: "POP",
            bytes: 1u8,
            cycles: vec!(12u8),
            operands: vec!(Operand {
                name: "BC",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC2",
            mnemonic: "JP",
            bytes: 3u8,
            cycles: vec!(16u8, 12u8),
            operands: vec!(
                Operand {
                    name: "NZ",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC3",
            mnemonic: "JP",
            bytes: 3u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "a16",
                immediate: true,
                bytes: 2u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC4",
            mnemonic: "CALL",
            bytes: 3u8,
            cycles: vec!(24u8, 12u8),
            operands: vec!(
                Operand {
                    name: "NZ",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC5",
            mnemonic: "PUSH",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "BC",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC6",
            mnemonic: "ADD",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xC7",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$00",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC8",
            mnemonic: "RET",
            bytes: 1u8,
            cycles: vec!(20u8, 8u8),
            operands: vec!(Operand {
                name: "Z",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC9",
            mnemonic: "RET",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCA",
            mnemonic: "JP",
            bytes: 3u8,
            cycles: vec!(16u8, 12u8),
            operands: vec!(
                Operand {
                    name: "Z",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCB",
            mnemonic: "PREFIX",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCC",
            mnemonic: "CALL",
            bytes: 3u8,
            cycles: vec!(24u8, 12u8),
            operands: vec!(
                Operand {
                    name: "Z",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCD",
            mnemonic: "CALL",
            bytes: 3u8,
            cycles: vec!(24u8),
            operands: vec!(Operand {
                name: "a16",
                immediate: true,
                bytes: 2u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCE",
            mnemonic: "ADC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xCF",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$08",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD0",
            mnemonic: "RET",
            bytes: 1u8,
            cycles: vec!(20u8, 8u8),
            operands: vec!(Operand {
                name: "NC",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD1",
            mnemonic: "POP",
            bytes: 1u8,
            cycles: vec!(12u8),
            operands: vec!(Operand {
                name: "DE",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD2",
            mnemonic: "JP",
            bytes: 3u8,
            cycles: vec!(16u8, 12u8),
            operands: vec!(
                Operand {
                    name: "NC",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD3",
            mnemonic: "ILLEGAL_D3",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD4",
            mnemonic: "CALL",
            bytes: 3u8,
            cycles: vec!(24u8, 12u8),
            operands: vec!(
                Operand {
                    name: "NC",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD5",
            mnemonic: "PUSH",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "DE",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD6",
            mnemonic: "SUB",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xD7",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$10",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD8",
            mnemonic: "RET",
            bytes: 1u8,
            cycles: vec!(20u8, 8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD9",
            mnemonic: "RETI",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDA",
            mnemonic: "JP",
            bytes: 3u8,
            cycles: vec!(16u8, 12u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDB",
            mnemonic: "ILLEGAL_DB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDC",
            mnemonic: "CALL",
            bytes: 3u8,
            cycles: vec!(24u8, 12u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: true,
                    bytes: 2u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDD",
            mnemonic: "ILLEGAL_DD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDE",
            mnemonic: "SBC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xDF",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$18",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE0",
            mnemonic: "LDH",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "a8",
                    immediate: false,
                    bytes: 1u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE1",
            mnemonic: "POP",
            bytes: 1u8,
            cycles: vec!(12u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE2",
            mnemonic: "LDH",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "C",
                    immediate: false,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE3",
            mnemonic: "ILLEGAL_E3",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE4",
            mnemonic: "ILLEGAL_E4",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE5",
            mnemonic: "PUSH",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE6",
            mnemonic: "AND",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "0",
            }
        },
        Opcode {
            name: "0xE7",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$20",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE8",
            mnemonic: "ADD",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "SP",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "e8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "0",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xE9",
            mnemonic: "JP",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEA",
            mnemonic: "LD",
            bytes: 3u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "a16",
                    immediate: false,
                    bytes: 2u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEB",
            mnemonic: "ILLEGAL_EB",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEC",
            mnemonic: "ILLEGAL_EC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xED",
            mnemonic: "ILLEGAL_ED",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEE",
            mnemonic: "XOR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xEF",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$28",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF0",
            mnemonic: "LDH",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a8",
                    immediate: false,
                    bytes: 1u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF1",
            mnemonic: "POP",
            bytes: 1u8,
            cycles: vec!(12u8),
            operands: vec!(Operand {
                name: "AF",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "N",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xF2",
            mnemonic: "LDH",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF3",
            mnemonic: "DI",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF4",
            mnemonic: "ILLEGAL_F4",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF5",
            mnemonic: "PUSH",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "AF",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF6",
            mnemonic: "OR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0xF7",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$30",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF8",
            mnemonic: "LD",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "SP",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "e8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "0",
                n: "0",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xF9",
            mnemonic: "LD",
            bytes: 1u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "SP",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFA",
            mnemonic: "LD",
            bytes: 3u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "a16",
                    immediate: false,
                    bytes: 2u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFB",
            mnemonic: "EI",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFC",
            mnemonic: "ILLEGAL_FC",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFD",
            mnemonic: "ILLEGAL_FD",
            bytes: 1u8,
            cycles: vec!(4u8),
            operands: vec!(),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFE",
            mnemonic: "CP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "n8",
                    immediate: true,
                    bytes: 1u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "1",
                h: "H",
                c: "C",
            }
        },
        Opcode {
            name: "0xFF",
            mnemonic: "RST",
            bytes: 1u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "$38",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
    ];
    static ref CBPREFIXED_OPCODES: [Opcode; 256usize] = [
        Opcode {
            name: "0x00",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x01",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x02",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x03",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x04",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x05",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x06",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x07",
            mnemonic: "RLC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x08",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x09",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x0A",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x0B",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x0C",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x0D",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x0E",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x0F",
            mnemonic: "RRC",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x10",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x11",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x12",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x13",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x14",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x15",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x16",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x17",
            mnemonic: "RL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x18",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x19",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x1A",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x1B",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x1C",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x1D",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x1E",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x1F",
            mnemonic: "RR",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x20",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x21",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x22",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x23",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x24",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x25",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x26",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x27",
            mnemonic: "SLA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x28",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x29",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x2A",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x2B",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x2C",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x2D",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x2E",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x2F",
            mnemonic: "SRA",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x30",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x31",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x32",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x33",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x34",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x35",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x36",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x37",
            mnemonic: "SWAP",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "0",
            }
        },
        Opcode {
            name: "0x38",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "B",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x39",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "C",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x3A",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "D",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x3B",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "E",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x3C",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "H",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x3D",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "L",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x3E",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(Operand {
                name: "HL",
                immediate: false,
                bytes: 0u8,
            },),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x3F",
            mnemonic: "SRL",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(Operand {
                name: "A",
                immediate: true,
                bytes: 0u8,
            },),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "0",
                c: "C",
            }
        },
        Opcode {
            name: "0x40",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x41",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x42",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x43",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x44",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x45",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x46",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x47",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x48",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x49",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x4A",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x4B",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x4C",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x4D",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x4E",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x4F",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x50",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x51",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x52",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x53",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x54",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x55",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x56",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x57",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x58",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x59",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x5A",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x5B",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x5C",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x5D",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x5E",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x5F",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x60",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x61",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x62",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x63",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x64",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x65",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x66",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x67",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x68",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x69",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x6A",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x6B",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x6C",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x6D",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x6E",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x6F",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x70",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x71",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x72",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x73",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x74",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x75",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x76",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x77",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x78",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x79",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x7A",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x7B",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x7C",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x7D",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x7E",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(12u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x7F",
            mnemonic: "BIT",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "Z",
                n: "0",
                h: "1",
                c: "-",
            }
        },
        Opcode {
            name: "0x80",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x81",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x82",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x83",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x84",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x85",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x86",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x87",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x88",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x89",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x8A",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x8B",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x8C",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x8D",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x8E",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x8F",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x90",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x91",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x92",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x93",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x94",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x95",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x96",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x97",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x98",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x99",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x9A",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x9B",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x9C",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x9D",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x9E",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0x9F",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA0",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA1",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA2",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA3",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA4",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA5",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA6",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA7",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA8",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xA9",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xAA",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xAB",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xAC",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xAD",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xAE",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xAF",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB0",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB1",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB2",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB3",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB4",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB5",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB6",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB7",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB8",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xB9",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xBA",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xBB",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xBC",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xBD",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xBE",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xBF",
            mnemonic: "RES",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC0",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC1",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC2",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC3",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC4",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC5",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC6",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC7",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "0",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC8",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xC9",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCA",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCB",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCC",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCD",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCE",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xCF",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "1",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD0",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD1",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD2",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD3",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD4",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD5",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD6",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD7",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "2",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD8",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xD9",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDA",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDB",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDC",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDD",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDE",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xDF",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "3",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE0",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE1",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE2",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE3",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE4",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE5",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE6",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE7",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "4",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE8",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xE9",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEA",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEB",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEC",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xED",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEE",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xEF",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "5",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF0",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF1",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF2",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF3",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF4",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF5",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF6",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF7",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "6",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF8",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "B",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xF9",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "C",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFA",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "D",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFB",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "E",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFC",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "H",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFD",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "L",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFE",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(16u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "HL",
                    immediate: false,
                    bytes: 0u8,
                },
            ),
            immediate: false,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
        Opcode {
            name: "0xFF",
            mnemonic: "SET",
            bytes: 2u8,
            cycles: vec!(8u8),
            operands: vec!(
                Operand {
                    name: "7",
                    immediate: true,
                    bytes: 0u8,
                },
                Operand {
                    name: "A",
                    immediate: true,
                    bytes: 0u8,
                },
            ),
            immediate: true,
            flags: Flag {
                z: "-",
                n: "-",
                h: "-",
                c: "-",
            }
        },
    ];
}
