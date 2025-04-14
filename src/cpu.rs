use crate::mmu::Mmu;
mod alu;
mod bits;
mod jumps;
mod loads;
mod misc;
pub mod registers;
use alu::{AluBinary, AluUnary};
use bits::{BitflagOp, BitshiftOp};
use registers::Flag;
use registers::Registers;
use registers::{R8, R16};

pub struct Cpu {
    pub reg: Registers,
    pub mmu: Mmu,
    pub instruction_tick_cycles: u8,
    ime: bool,
    ime_pending: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),
            mmu: Mmu::new(),
            instruction_tick_cycles: 0,
            ime: true,
            ime_pending: false,
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let pc = self.reg.get16(R16::PC);
        let byte = self.mmu.read_byte(pc);

        let next_addr = pc + 1;
        self.reg.set16(R16::PC, next_addr);
        byte
    }

    pub fn fetch_word(&mut self) -> u16 {
        let pc = self.reg.get16(R16::PC);
        let word = self.mmu.read_word(pc);

        let next_addr = pc + 2;
        self.reg.set16(R16::PC, next_addr);
        word
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_byte();
        println!("{:02x}", opcode);

        // Look up the number of clock cycles this instruction will take
        // Note: In the case of checked condition functions, the minimum
        // time is assumed. The functions will increment adjust the value
        // when called if the condition is met.
        self.instruction_tick_cycles = 0; // TODO: Index clock cycle array

        // Every instruction that contains an n8, a8, or e8 will fetch a byte.
        // Every instruction that contains an n16 or a16 will fetch a word.

        match opcode {
            0x00 => (),                                       // NOP
            0x01 => self.ld_r16_n16(R16::BC),                 // LD BC, n16
            0x02 => self.ld_at_r16_a(R16::BC),                // LD BC, A
            0x03 => self.inc_r16(R16::BC),                    // INC BC
            0x04 => self.alu_r8(AluUnary::Inc, R8::B),        // INC B
            0x05 => self.alu_r8(AluUnary::Dec, R8::B),        // DEC B
            0x06 => self.ld_r8_n8(R8::B),                     // LD B, n8
            0x07 => self.bitshift_r8(BitshiftOp::Rrc, R8::A), // RLCA
            0x08 => self.ld_at_n16_sp(),                      // LD [n16], SP
            0x09 => self.add_hl_r16(R16::BC),                 // ADD HL, BC
            0x0A => self.ld_a_at_r16(R16::BC),                // LD A, [BC]
            0x0B => self.dec_r16(R16::BC),                    // DEC BC
            0x0C => self.alu_r8(AluUnary::Inc, R8::C),        // INC C
            0x0D => self.alu_r8(AluUnary::Dec, R8::C),        // DEC C
            0x0E => self.ld_r8_n8(R8::C),                     // LD C, n8
            0x0F => self.bitshift_r8(BitshiftOp::Rrc, R8::A), // RRCA

            0x10 => todo!("STOP n8"),                        // STOP n8
            0x11 => self.ld_r16_n16(R16::DE),                // LD DE, n16
            0x12 => self.ld_at_r16_a(R16::DE),               // LD DE, A
            0x13 => self.inc_r16(R16::DE),                   // INC DE
            0x14 => self.alu_r8(AluUnary::Inc, R8::D),       // INC D
            0x15 => self.alu_r8(AluUnary::Dec, R8::D),       // DEC D
            0x16 => self.ld_r8_n8(R8::D),                    // LD D, n8
            0x17 => self.bitshift_r8(BitshiftOp::Rl, R8::A), // RLA
            0x18 => self.jr_e8(),                            // JR e8
            0x19 => self.add_hl_r16(R16::DE),                // ADD HL, DE
            0x1A => self.ld_a_at_r16(R16::DE),               // LD A, [DE]
            0x1B => self.dec_r16(R16::DE),                   // DEC DE
            0x1C => self.alu_r8(AluUnary::Inc, R8::E),       // INC E
            0x1D => self.alu_r8(AluUnary::Dec, R8::E),       // DEC E
            0x1E => self.ld_r8_n8(R8::E),                    // LD E, n8
            0x1F => self.bitshift_r8(BitshiftOp::Rr, R8::A), // RRA

            0x20 => self.jr_cc_e8(Flag::Z, false), // JR NZ, e8
            0x21 => self.ld_r16_n16(R16::HL),                      // LD HL, n16
            0x22 => self.ld_at_hli_a(),                            // LD [HL+], A
            0x23 => self.inc_r16(R16::HL),                         // INC HL
            0x24 => self.alu_r8(AluUnary::Inc, R8::H),             // INC H
            0x25 => self.alu_r8(AluUnary::Dec, R8::H),             // DEC H
            0x26 => self.ld_r8_n8(R8::H),                          // LD H, n8
            0x27 => self.daa(),                                    // DAA
            0x28 => self.jr_cc_e8(Flag::Z, true),                  // JR Z e8
            0x29 => self.add_hl_r16(R16::HL),                      // ADD HL, HL
            0x2A => self.ld_a_at_hli(),                            // LD A, [HL+]
            0x2B => self.dec_r16(R16::HL),                         // DEC HL
            0x2C => self.alu_r8(AluUnary::Inc, R8::L),             // INC L
            0x2D => self.alu_r8(AluUnary::Dec, R8::L),             // DEC L
            0x2E => self.ld_r8_n8(R8::L),                          // LD L, n8
            0x2F => self.cpl(),                                    // CPL

            0x30 => self.jr_cc_e8(Flag::C, false), // JR NC, e8
            0x31 => self.ld_r16_n16(R16::SP),      // LD SP, n16
            0x32 => self.ld_at_hld_a(),            // LD [HL-], A
            0x33 => self.inc_r16(R16::SP),         // INC SP
            0x34 => self.alu_at_hl(AluUnary::Inc), // INC [HL]
            0x35 => self.alu_at_hl(AluUnary::Dec), // DEC [HL]
            0x36 => self.ld_at_hl_n8(),            // LD [HL], n8
            0x37 => self.scf(),                    // SCF
            0x38 => self.jr_cc_e8(Flag::C, true),  // JR C, e8
            0x39 => self.add_hl_r16(R16::SP),      // ADD HL, SP
            0x3A => self.ld_a_at_hld(),            // LD A, [HL-]
            0x3B => self.dec_r16(R16::SP),         // DEC SP
            0x3C => self.alu_r8(AluUnary::Inc, R8::A), // INC A
            0x3D => self.alu_r8(AluUnary::Dec, R8::A), // DEC A
            0x3E => self.ld_r8_n8(R8::A),          // LD A, n8
            0x3F => self.ccf(),                    // CCF

            0x40 => self.ld_r8_r8(R8::B, R8::B), // LD B, B
            0x41 => self.ld_r8_r8(R8::B, R8::C), // LD B, C
            0x42 => self.ld_r8_r8(R8::B, R8::D), // LD B, D
            0x43 => self.ld_r8_r8(R8::B, R8::E), // LD B, E
            0x44 => self.ld_r8_r8(R8::B, R8::H), // LD B, H
            0x45 => self.ld_r8_r8(R8::B, R8::L), // LD B, L
            0x46 => self.ld_r8_at_hl(R8::B),     // LD B, [HL]
            0x47 => self.ld_r8_r8(R8::B, R8::A), // LD B, A
            0x48 => self.ld_r8_r8(R8::C, R8::B), // LD C, B
            0x49 => self.ld_r8_r8(R8::C, R8::C), // LD C, C
            0x4A => self.ld_r8_r8(R8::C, R8::D), // LD C, D
            0x4B => self.ld_r8_r8(R8::C, R8::E), // LD C, E
            0x4C => self.ld_r8_r8(R8::C, R8::H), // LD C, H
            0x4D => self.ld_r8_r8(R8::C, R8::L), // LD C, L
            0x4E => self.ld_r8_at_hl(R8::C),     // LD C, [HL]
            0x4F => self.ld_r8_r8(R8::C, R8::A), // LD C, A

            0x50 => self.ld_r8_r8(R8::D, R8::B), // LD D, B
            0x51 => self.ld_r8_r8(R8::D, R8::C), // LD D, C
            0x52 => self.ld_r8_r8(R8::D, R8::D), // LD D, D
            0x53 => self.ld_r8_r8(R8::D, R8::E), // LD D, E
            0x54 => self.ld_r8_r8(R8::D, R8::H), // LD D, H
            0x55 => self.ld_r8_r8(R8::D, R8::L), // LD D, L
            0x56 => self.ld_r8_at_hl(R8::D),     // LD D, [HL]
            0x57 => self.ld_r8_r8(R8::D, R8::A), // LD D, A
            0x58 => self.ld_r8_r8(R8::E, R8::B), // LD E, B
            0x59 => self.ld_r8_r8(R8::E, R8::C), // LD E, C
            0x5A => self.ld_r8_r8(R8::E, R8::D), // LD E, D
            0x5B => self.ld_r8_r8(R8::E, R8::E), // LD E, E
            0x5C => self.ld_r8_r8(R8::E, R8::H), // LD E, H
            0x5D => self.ld_r8_r8(R8::E, R8::L), // LD E, L
            0x5E => self.ld_r8_at_hl(R8::E),     // LD E, [HL]
            0x5F => self.ld_r8_r8(R8::E, R8::A), // LD E, A

            0x60 => self.ld_r8_r8(R8::H, R8::B), // LD H, B
            0x61 => self.ld_r8_r8(R8::H, R8::C), // LD H, C
            0x62 => self.ld_r8_r8(R8::H, R8::D), // LD H, D
            0x63 => self.ld_r8_r8(R8::H, R8::E), // LD H, E
            0x64 => self.ld_r8_r8(R8::H, R8::H), // LD H, H
            0x65 => self.ld_r8_r8(R8::H, R8::L), // LD H, L
            0x66 => self.ld_r8_at_hl(R8::H),     // LD H, [HL]
            0x67 => self.ld_r8_r8(R8::H, R8::A), // LD H, A
            0x68 => self.ld_r8_r8(R8::L, R8::B), // LD L, B
            0x69 => self.ld_r8_r8(R8::L, R8::C), // LD L, C
            0x6A => self.ld_r8_r8(R8::L, R8::D), // LD L, D
            0x6B => self.ld_r8_r8(R8::L, R8::E), // LD L, E
            0x6C => self.ld_r8_r8(R8::L, R8::H), // LD L, H
            0x6D => self.ld_r8_r8(R8::L, R8::L), // LD L, L
            0x6E => self.ld_r8_at_hl(R8::L),     // LD L, [HL]
            0x6F => self.ld_r8_r8(R8::L, R8::A), // LD L, A

            0x70 => self.ld_at_hl_r8(R8::B),     // LD [HL], B
            0x71 => self.ld_at_hl_r8(R8::C),     // LD [HL], C
            0x72 => self.ld_at_hl_r8(R8::D),     // LD [HL], D
            0x73 => self.ld_at_hl_r8(R8::E),     // LD [HL], E
            0x74 => self.ld_at_hl_r8(R8::H),     // LD [HL], H
            0x75 => self.ld_at_hl_r8(R8::L),     // LD [HL], L
            0x76 => todo!("HALT"),               // HALT
            0x77 => self.ld_at_hl_r8(R8::A),     // LD [HL], A
            0x78 => self.ld_r8_r8(R8::A, R8::B), // LD A, B
            0x79 => self.ld_r8_r8(R8::A, R8::C), // LD A, C
            0x7A => self.ld_r8_r8(R8::A, R8::D), // LD A, D
            0x7B => self.ld_r8_r8(R8::A, R8::E), // LD A, E
            0x7C => self.ld_r8_r8(R8::A, R8::H), // LD A, H
            0x7D => self.ld_r8_r8(R8::A, R8::L), // LD A, L
            0x7E => self.ld_r8_at_hl(R8::A),     // LD A, [HL]
            0x7F => self.ld_r8_r8(R8::A, R8::A), // LD A, A

            0x80 => self.alu_a_r8(AluBinary::Add, R8::B), // ADD A, B
            0x81 => self.alu_a_r8(AluBinary::Add, R8::C), // ADD A, C
            0x82 => self.alu_a_r8(AluBinary::Add, R8::D), // ADD A, D
            0x83 => self.alu_a_r8(AluBinary::Add, R8::E), // ADD A, E
            0x84 => self.alu_a_r8(AluBinary::Add, R8::H), // ADD A, H
            0x85 => self.alu_a_r8(AluBinary::Add, R8::L), // ADD A, L
            0x86 => self.alu_a_at_hl(AluBinary::Add),     // ADD A, [HL]
            0x87 => self.alu_a_r8(AluBinary::Add, R8::A), // ADD A, A
            0x88 => self.alu_a_r8(AluBinary::Adc, R8::B), // ADC A, B
            0x89 => self.alu_a_r8(AluBinary::Adc, R8::C), // ADC A, C
            0x8A => self.alu_a_r8(AluBinary::Adc, R8::D), // ADC A, D
            0x8B => self.alu_a_r8(AluBinary::Adc, R8::E), // ADC A, E
            0x8C => self.alu_a_r8(AluBinary::Adc, R8::H), // ADC A, H
            0x8D => self.alu_a_r8(AluBinary::Adc, R8::L), // ADC A, L
            0x8E => self.alu_a_at_hl(AluBinary::Adc),     // ADC A, [HL]
            0x8F => self.alu_a_r8(AluBinary::Adc, R8::A), // ADC A, A

            0x90 => self.alu_a_r8(AluBinary::Sub, R8::B), // SUB A, B
            0x91 => self.alu_a_r8(AluBinary::Sub, R8::C), // SUB A, C
            0x92 => self.alu_a_r8(AluBinary::Sub, R8::D), // SUB A, D
            0x93 => self.alu_a_r8(AluBinary::Sub, R8::E), // SUB A, E
            0x94 => self.alu_a_r8(AluBinary::Sub, R8::H), // SUB A, H
            0x95 => self.alu_a_r8(AluBinary::Sub, R8::L), // SUB A, L
            0x96 => self.alu_a_at_hl(AluBinary::Sub),     // SUB A, [HL]
            0x97 => self.alu_a_r8(AluBinary::Sub, R8::A), // SUB A, A
            0x98 => self.alu_a_r8(AluBinary::Sbc, R8::B), // SBC A, B
            0x99 => self.alu_a_r8(AluBinary::Sbc, R8::C), // SBC A, C
            0x9A => self.alu_a_r8(AluBinary::Sbc, R8::D), // SBC A, D
            0x9B => self.alu_a_r8(AluBinary::Sbc, R8::E), // SBC A, E
            0x9C => self.alu_a_r8(AluBinary::Sbc, R8::H), // SBC A, H
            0x9D => self.alu_a_r8(AluBinary::Sbc, R8::L), // SBC A, L
            0x9E => self.alu_a_at_hl(AluBinary::Sbc),     // SBC A, [HL]
            0x9F => self.alu_a_r8(AluBinary::Sbc, R8::A), // SBC A, A

            0xA0 => self.alu_a_r8(AluBinary::And, R8::B), // AND A, B
            0xA1 => self.alu_a_r8(AluBinary::And, R8::C), // AND A, C
            0xA2 => self.alu_a_r8(AluBinary::And, R8::D), // AND A, D
            0xA3 => self.alu_a_r8(AluBinary::And, R8::E), // AND A, E
            0xA4 => self.alu_a_r8(AluBinary::And, R8::H), // AND A, H
            0xA5 => self.alu_a_r8(AluBinary::And, R8::L), // AND A, L
            0xA6 => self.alu_a_at_hl(AluBinary::And),     // AND A, [HL]
            0xA7 => self.alu_a_r8(AluBinary::And, R8::A), // AND A, A
            0xA8 => self.alu_a_r8(AluBinary::Xor, R8::B), // XOR A, B
            0xA9 => self.alu_a_r8(AluBinary::Xor, R8::C), // XOR A, C
            0xAA => self.alu_a_r8(AluBinary::Xor, R8::D), // XOR A, D
            0xAB => self.alu_a_r8(AluBinary::Xor, R8::E), // XOR A, E
            0xAC => self.alu_a_r8(AluBinary::Xor, R8::H), // XOR A, H
            0xAD => self.alu_a_r8(AluBinary::Xor, R8::L), // XOR A, L
            0xAE => self.alu_a_at_hl(AluBinary::Xor),     // XOR A, [HL]
            0xAF => self.alu_a_r8(AluBinary::Xor, R8::A), // XOR A, A

            0xB0 => self.alu_a_r8(AluBinary::Or, R8::B), // OR A, B
            0xB1 => self.alu_a_r8(AluBinary::Or, R8::C), // OR A, C
            0xB2 => self.alu_a_r8(AluBinary::Or, R8::D), // OR A, D
            0xB3 => self.alu_a_r8(AluBinary::Or, R8::E), // OR A, E
            0xB4 => self.alu_a_r8(AluBinary::Or, R8::H), // OR A, H
            0xB5 => self.alu_a_r8(AluBinary::Or, R8::L), // OR A, L
            0xB6 => self.alu_a_at_hl(AluBinary::Or),     // OR A, [HL]
            0xB7 => self.alu_a_r8(AluBinary::Or, R8::A), // OR A, A
            0xB8 => self.alu_a_r8(AluBinary::Cp, R8::B), // CP A, B
            0xB9 => self.alu_a_r8(AluBinary::Cp, R8::C), // CP A, C
            0xBA => self.alu_a_r8(AluBinary::Cp, R8::D), // CP A, D
            0xBB => self.alu_a_r8(AluBinary::Cp, R8::E), // CP A, E
            0xBC => self.alu_a_r8(AluBinary::Cp, R8::H), // CP A, H
            0xBD => self.alu_a_r8(AluBinary::Cp, R8::L), // CP A, L
            0xBE => self.alu_a_at_hl(AluBinary::Cp),     // CP A, [HL]
            0xBF => self.alu_a_r8(AluBinary::Cp, R8::A), // CP A, A

            0xC0 => self.ret_cc(Flag::Z, false),      // RET NZ
            0xC1 => self.pop_r16(R16::BC),            // POP BC
            0xC2 => self.jp_cc_a16(Flag::Z, false),   // JP NZ
            0xC3 => self.jp_a16(),                    // JP a16
            0xC4 => self.call_cc_a16(Flag::Z, false), // CALL NZ, a16
            0xC5 => self.push_r16(R16::BC),           // PUSH BC
            0xC6 => self.alu_a_n8(AluBinary::Add),    // ADD A, n8
            0xC7 => self.rst_vec(0x00),               // RST $00
            0xC8 => self.ret_cc(Flag::Z, true),       // RET Z
            0xC9 => self.ret(),                       // RET
            0xCA => self.jp_cc_a16(Flag::Z, true),    // JP Z, a16
            0xCB => self.execute_prefixed(),          // PREFIX
            0xCC => self.call_cc_a16(Flag::Z, true),  // CALL Z, a16
            0xCD => self.call_a16(),                  // CALL a16
            0xCE => self.alu_a_n8(AluBinary::Adc),    // ADC A, n8
            0xCF => self.rst_vec(0x08),               // RST $08

            0xD0 => self.ret_cc(Flag::C, false),      // RET NC
            0xD1 => self.pop_r16(R16::DE),            // POP DE
            0xD2 => self.jp_cc_a16(Flag::C, false),   // JP NC, a16
            0xD3 => panic!("Invalid Instruction"),    // ---
            0xD4 => self.call_cc_a16(Flag::C, false), // CALL NC, a16
            0xD5 => self.push_r16(R16::DE),           // PUSH DE
            0xD6 => self.alu_a_n8(AluBinary::Sub),    // SUB A, n8
            0xD7 => self.rst_vec(0x10),               // RST $10
            0xD8 => self.ret_cc(Flag::C, true),       // RET C
            0xD9 => self.reti(),                      // RETI
            0xDA => self.jp_cc_a16(Flag::C, true),    // JP C, a16
            0xDB => panic!("Invalid Instruction"),    // ---
            0xDC => self.call_cc_a16(Flag::C, true),  // CALL C, a16
            0xDD => panic!("Invalid Instruction"),    // ---
            0xDE => self.alu_a_n8(AluBinary::Sbc),    // SBC A, n8
            0xDF => self.rst_vec(0x18),               // RST $18

            0xE0 => self.ldh_at_a8_a(),            // LDH [a8], A
            0xE1 => self.pop_r16(R16::HL),         // POP HL
            0xE2 => self.ldh_c_a(),                // LDH [C], A
            0xE3 => panic!("Invalid Instruction"), // ---
            0xE4 => panic!("Invalid Instruction"), // ---
            0xE5 => self.push_r16(R16::HL),        // PUSH HL
            0xE6 => self.alu_a_n8(AluBinary::And), // AND A, N8
            0xE7 => self.rst_vec(0x20),            // RST $20
            0xE8 => self.add_sp_e8(),              // ADD SP, e8
            0xE9 => self.jp_hl(),                  // JP HL
            0xEA => self.ld_at_a16_a(),            // LD [a16], A
            0xEB => panic!("Invalid Instruction"), // ---
            0xEC => panic!("Invalid Instruction"), // ---
            0xED => panic!("Invalid Instruction"), // ---
            0xEE => self.alu_a_n8(AluBinary::Xor), // XOR A, n8
            0xEF => self.rst_vec(0x28),            // RST $28

            0xF0 => self.ldh_a_a8(),               // LDH A, [a8]
            0xF1 => self.pop_r16(R16::AF),         // POP AF
            0xF2 => self.ldh_a_at_c(),             // LDH A, [C]
            0xF3 => self.di(),                     // DI
            0xF4 => panic!("Invalid Instruction"), // ---
            0xF5 => self.push_r16(R16::AF),        // PUSH AF
            0xF6 => self.alu_a_n8(AluBinary::Or),  // OR A, n8
            0xF7 => self.rst_vec(0x30),            // RST $30
            0xF8 => self.ld_hl_sp_e8(),            // LD HL, SP + e8
            0xF9 => self.ld_sp_hl(),               // LD SP, HL
            0xFA => self.ld_a_at_a16(),            // LD A, [a16]
            0xFB => self.ei(),                     // EI
            0xFC => panic!("Invalid Instruction"), // ---
            0xFD => panic!("Invalid Instruction"), // ---
            0xFE => self.alu_a_n8(AluBinary::Cp),  // CP A, n8
            0xFF => self.rst_vec(0x38),            // RST $38
        }
    }

    fn execute_prefixed(&mut self) {
        let opcode = self.fetch_byte();

        match opcode {
            0x00 => self.bitshift_r8(BitshiftOp::Rlc, R8::B), // RLC B
            0x01 => self.bitshift_r8(BitshiftOp::Rlc, R8::C), // RLC C
            0x02 => self.bitshift_r8(BitshiftOp::Rlc, R8::D), // RLC D
            0x03 => self.bitshift_r8(BitshiftOp::Rlc, R8::E), // RLC E
            0x04 => self.bitshift_r8(BitshiftOp::Rlc, R8::H), // RLC H
            0x05 => self.bitshift_r8(BitshiftOp::Rlc, R8::L), // RLC L
            0x06 => self.bitshift_at_hl(BitshiftOp::Rlc),     // RLC [HL]
            0x07 => self.bitshift_r8(BitshiftOp::Rlc, R8::A), // RLC A
            0x08 => self.bitshift_r8(BitshiftOp::Rrc, R8::B), // RRC B
            0x09 => self.bitshift_r8(BitshiftOp::Rrc, R8::C), // RRC C
            0x0A => self.bitshift_r8(BitshiftOp::Rrc, R8::D), // RRC D
            0x0B => self.bitshift_r8(BitshiftOp::Rrc, R8::E), // RRC E
            0x0C => self.bitshift_r8(BitshiftOp::Rrc, R8::H), // RRC H
            0x0D => self.bitshift_r8(BitshiftOp::Rrc, R8::L), // RRC L
            0x0E => self.bitshift_at_hl(BitshiftOp::Rrc),     // RRC [HL]
            0x0F => self.bitshift_r8(BitshiftOp::Rrc, R8::A), // RRC A

            0x10 => self.bitshift_r8(BitshiftOp::Rl, R8::B), // RL B
            0x11 => self.bitshift_r8(BitshiftOp::Rl, R8::C), // RL C
            0x12 => self.bitshift_r8(BitshiftOp::Rl, R8::D), // RL D
            0x13 => self.bitshift_r8(BitshiftOp::Rl, R8::E), // RL E
            0x14 => self.bitshift_r8(BitshiftOp::Rl, R8::H), // RL H
            0x15 => self.bitshift_r8(BitshiftOp::Rl, R8::L), // RL L
            0x16 => self.bitshift_at_hl(BitshiftOp::Rl),     // RL [HL]
            0x17 => self.bitshift_r8(BitshiftOp::Rl, R8::A), // RL A
            0x18 => self.bitshift_r8(BitshiftOp::Rr, R8::B), // RR B
            0x19 => self.bitshift_r8(BitshiftOp::Rr, R8::C), // RR C
            0x1A => self.bitshift_r8(BitshiftOp::Rr, R8::D), // RR D
            0x1B => self.bitshift_r8(BitshiftOp::Rr, R8::E), // RR E
            0x1C => self.bitshift_r8(BitshiftOp::Rr, R8::H), // RR H
            0x1D => self.bitshift_r8(BitshiftOp::Rr, R8::L), // RR L
            0x1E => self.bitshift_at_hl(BitshiftOp::Rr),     // RR [HL]
            0x1F => self.bitshift_r8(BitshiftOp::Rr, R8::A), // RR A

            0x20 => self.bitshift_r8(BitshiftOp::Sla, R8::B), // SLA B
            0x21 => self.bitshift_r8(BitshiftOp::Sla, R8::C), // SLA C
            0x22 => self.bitshift_r8(BitshiftOp::Sla, R8::D), // SLA D
            0x23 => self.bitshift_r8(BitshiftOp::Sla, R8::E), // SLA E
            0x24 => self.bitshift_r8(BitshiftOp::Sla, R8::H), // SLA H
            0x25 => self.bitshift_r8(BitshiftOp::Sla, R8::L), // SLA L
            0x26 => self.bitshift_at_hl(BitshiftOp::Sla),     // SLA [HL]
            0x27 => self.bitshift_r8(BitshiftOp::Sla, R8::A), // SLA A
            0x28 => self.bitshift_r8(BitshiftOp::Sra, R8::B), // SRA B
            0x29 => self.bitshift_r8(BitshiftOp::Sra, R8::C), // SRA C
            0x2A => self.bitshift_r8(BitshiftOp::Sra, R8::D), // SRA D
            0x2B => self.bitshift_r8(BitshiftOp::Sra, R8::E), // SRA E
            0x2C => self.bitshift_r8(BitshiftOp::Sra, R8::H), // SRA H
            0x2D => self.bitshift_r8(BitshiftOp::Sra, R8::L), // SRA L
            0x2E => self.bitshift_at_hl(BitshiftOp::Sra),     // SRA [HL]
            0x2F => self.bitshift_r8(BitshiftOp::Sra, R8::A), // SRA A

            0x30 => self.bitshift_r8(BitshiftOp::Swap, R8::B), // SWAP B
            0x31 => self.bitshift_r8(BitshiftOp::Swap, R8::C), // SWAP C
            0x32 => self.bitshift_r8(BitshiftOp::Swap, R8::D), // SWAP D
            0x33 => self.bitshift_r8(BitshiftOp::Swap, R8::E), // SWAP E
            0x34 => self.bitshift_r8(BitshiftOp::Swap, R8::H), // SWAP H
            0x35 => self.bitshift_r8(BitshiftOp::Swap, R8::L), // SWAP L
            0x36 => self.bitshift_at_hl(BitshiftOp::Swap),     // SWAP [HL]
            0x37 => self.bitshift_r8(BitshiftOp::Swap, R8::A), // SWAP A
            0x38 => self.bitshift_r8(BitshiftOp::Srl, R8::B),  // SRL B
            0x39 => self.bitshift_r8(BitshiftOp::Srl, R8::C),  // SRL C
            0x3A => self.bitshift_r8(BitshiftOp::Srl, R8::D),  // SRL D
            0x3B => self.bitshift_r8(BitshiftOp::Srl, R8::E),  // SRL E
            0x3C => self.bitshift_r8(BitshiftOp::Srl, R8::H),  // SRL H
            0x3D => self.bitshift_r8(BitshiftOp::Srl, R8::L),  // SRL L
            0x3E => self.bitshift_at_hl(BitshiftOp::Srl),      // SRL [HL]
            0x3F => self.bitshift_r8(BitshiftOp::Srl, R8::A),  // SRL A

            0x40 => self.bitflag_u3_r8(BitflagOp::Bit, 0, R8::B), // BIT 0, B
            0x41 => self.bitflag_u3_r8(BitflagOp::Bit, 0, R8::C), // BIT 0, C
            0x42 => self.bitflag_u3_r8(BitflagOp::Bit, 0, R8::D), // BIT 0, D
            0x43 => self.bitflag_u3_r8(BitflagOp::Bit, 0, R8::E), // BIT 0, E
            0x44 => self.bitflag_u3_r8(BitflagOp::Bit, 0, R8::H), // BIT 0, H
            0x45 => self.bitflag_u3_r8(BitflagOp::Bit, 0, R8::L), // BIT 0, L
            0x46 => self.bitflag_u3_at_hl(BitflagOp::Bit, 0),     // BIT 0, [HL]
            0x47 => self.bitflag_u3_r8(BitflagOp::Bit, 0, R8::A), // BIT 0, A
            0x48 => self.bitflag_u3_r8(BitflagOp::Bit, 1, R8::B), // BIT 1, B
            0x49 => self.bitflag_u3_r8(BitflagOp::Bit, 1, R8::C), // BIT 1, C
            0x4A => self.bitflag_u3_r8(BitflagOp::Bit, 1, R8::D), // BIT 1, D
            0x4B => self.bitflag_u3_r8(BitflagOp::Bit, 1, R8::E), // BIT 1, E
            0x4C => self.bitflag_u3_r8(BitflagOp::Bit, 1, R8::H), // BIT 1, H
            0x4D => self.bitflag_u3_r8(BitflagOp::Bit, 1, R8::L), // BIT 1, L
            0x4E => self.bitflag_u3_at_hl(BitflagOp::Bit, 1),     // BIT 1, [HL]
            0x4F => self.bitflag_u3_r8(BitflagOp::Bit, 1, R8::A), // BIT 1, A

            0x50 => self.bitflag_u3_r8(BitflagOp::Bit, 2, R8::B), // BIT 2, B
            0x51 => self.bitflag_u3_r8(BitflagOp::Bit, 2, R8::C), // BIT 2, C
            0x52 => self.bitflag_u3_r8(BitflagOp::Bit, 2, R8::D), // BIT 2, D
            0x53 => self.bitflag_u3_r8(BitflagOp::Bit, 2, R8::E), // BIT 2, E
            0x54 => self.bitflag_u3_r8(BitflagOp::Bit, 2, R8::H), // BIT 2, H
            0x55 => self.bitflag_u3_r8(BitflagOp::Bit, 2, R8::L), // BIT 2, L
            0x56 => self.bitflag_u3_at_hl(BitflagOp::Bit, 2),     // BIT 2, [HL]
            0x57 => self.bitflag_u3_r8(BitflagOp::Bit, 2, R8::A), // BIT 2, A
            0x58 => self.bitflag_u3_r8(BitflagOp::Bit, 3, R8::B), // BIT 3, B
            0x59 => self.bitflag_u3_r8(BitflagOp::Bit, 3, R8::C), // BIT 3, C
            0x5A => self.bitflag_u3_r8(BitflagOp::Bit, 3, R8::D), // BIT 3, D
            0x5B => self.bitflag_u3_r8(BitflagOp::Bit, 3, R8::E), // BIT 3, E
            0x5C => self.bitflag_u3_r8(BitflagOp::Bit, 3, R8::H), // BIT 3, H
            0x5D => self.bitflag_u3_r8(BitflagOp::Bit, 3, R8::L), // BIT 3, L
            0x5E => self.bitflag_u3_at_hl(BitflagOp::Bit, 3),     // BIT 3, [HL]
            0x5F => self.bitflag_u3_r8(BitflagOp::Bit, 3, R8::A), // BIT 3, A

            0x60 => self.bitflag_u3_r8(BitflagOp::Bit, 4, R8::B), // BIT 4, B
            0x61 => self.bitflag_u3_r8(BitflagOp::Bit, 4, R8::C), // BIT 4, C
            0x62 => self.bitflag_u3_r8(BitflagOp::Bit, 4, R8::D), // BIT 4, D
            0x63 => self.bitflag_u3_r8(BitflagOp::Bit, 4, R8::E), // BIT 4, E
            0x64 => self.bitflag_u3_r8(BitflagOp::Bit, 4, R8::H), // BIT 4, H
            0x65 => self.bitflag_u3_r8(BitflagOp::Bit, 4, R8::L), // BIT 4, L
            0x66 => self.bitflag_u3_at_hl(BitflagOp::Bit, 4),     // BIT 4, [HL]
            0x67 => self.bitflag_u3_r8(BitflagOp::Bit, 4, R8::A), // BIT 4, A
            0x68 => self.bitflag_u3_r8(BitflagOp::Bit, 5, R8::B), // BIT 5, B
            0x69 => self.bitflag_u3_r8(BitflagOp::Bit, 5, R8::C), // BIT 5, C
            0x6A => self.bitflag_u3_r8(BitflagOp::Bit, 5, R8::D), // BIT 5, D
            0x6B => self.bitflag_u3_r8(BitflagOp::Bit, 5, R8::E), // BIT 5, E
            0x6C => self.bitflag_u3_r8(BitflagOp::Bit, 5, R8::H), // BIT 5, H
            0x6D => self.bitflag_u3_r8(BitflagOp::Bit, 5, R8::L), // BIT 5, L
            0x6E => self.bitflag_u3_at_hl(BitflagOp::Bit, 5),     // BIT 5, [HL]
            0x6F => self.bitflag_u3_r8(BitflagOp::Bit, 5, R8::A), // BIT 5, A

            0x70 => self.bitflag_u3_r8(BitflagOp::Bit, 6, R8::B), // BIT 6, B
            0x71 => self.bitflag_u3_r8(BitflagOp::Bit, 6, R8::C), // BIT 6, C
            0x72 => self.bitflag_u3_r8(BitflagOp::Bit, 6, R8::D), // BIT 6, D
            0x73 => self.bitflag_u3_r8(BitflagOp::Bit, 6, R8::E), // BIT 6, E
            0x74 => self.bitflag_u3_r8(BitflagOp::Bit, 6, R8::H), // BIT 6, H
            0x75 => self.bitflag_u3_r8(BitflagOp::Bit, 6, R8::L), // BIT 6, L
            0x76 => self.bitflag_u3_at_hl(BitflagOp::Bit, 6),     // BIT 6, [HL]
            0x77 => self.bitflag_u3_r8(BitflagOp::Bit, 6, R8::A), // BIT 6, A
            0x78 => self.bitflag_u3_r8(BitflagOp::Bit, 7, R8::B), // BIT 7, B
            0x79 => self.bitflag_u3_r8(BitflagOp::Bit, 7, R8::C), // BIT 7, C
            0x7A => self.bitflag_u3_r8(BitflagOp::Bit, 7, R8::D), // BIT 7, D
            0x7B => self.bitflag_u3_r8(BitflagOp::Bit, 7, R8::E), // BIT 7, E
            0x7C => self.bitflag_u3_r8(BitflagOp::Bit, 7, R8::H), // BIT 7, H
            0x7D => self.bitflag_u3_r8(BitflagOp::Bit, 7, R8::L), // BIT 7, L
            0x7E => self.bitflag_u3_at_hl(BitflagOp::Bit, 7),     // BIT 7, [HL]
            0x7F => self.bitflag_u3_r8(BitflagOp::Bit, 7, R8::A), // BIT 7, A

            0x80 => self.bitflag_u3_r8(BitflagOp::Res, 0, R8::B), // RES 0, B
            0x81 => self.bitflag_u3_r8(BitflagOp::Res, 0, R8::C), // RES 0, C
            0x82 => self.bitflag_u3_r8(BitflagOp::Res, 0, R8::D), // RES 0, D
            0x83 => self.bitflag_u3_r8(BitflagOp::Res, 0, R8::E), // RES 0, E
            0x84 => self.bitflag_u3_r8(BitflagOp::Res, 0, R8::H), // RES 0, H
            0x85 => self.bitflag_u3_r8(BitflagOp::Res, 0, R8::L), // RES 0, L
            0x86 => self.bitflag_u3_at_hl(BitflagOp::Res, 0),     // RES 0, [HL]
            0x87 => self.bitflag_u3_r8(BitflagOp::Res, 0, R8::A), // RES 0, A
            0x88 => self.bitflag_u3_r8(BitflagOp::Res, 1, R8::B), // RES 1, B
            0x89 => self.bitflag_u3_r8(BitflagOp::Res, 1, R8::C), // RES 1, C
            0x8A => self.bitflag_u3_r8(BitflagOp::Res, 1, R8::D), // RES 1, D
            0x8B => self.bitflag_u3_r8(BitflagOp::Res, 1, R8::E), // RES 1, E
            0x8C => self.bitflag_u3_r8(BitflagOp::Res, 1, R8::H), // RES 1, H
            0x8D => self.bitflag_u3_r8(BitflagOp::Res, 1, R8::L), // RES 1, L
            0x8E => self.bitflag_u3_at_hl(BitflagOp::Res, 1),     // RES 1, [HL]
            0x8F => self.bitflag_u3_r8(BitflagOp::Res, 1, R8::A), // RES 1, A

            0x90 => self.bitflag_u3_r8(BitflagOp::Res, 2, R8::B), // RES 2, B
            0x91 => self.bitflag_u3_r8(BitflagOp::Res, 2, R8::C), // RES 2, C
            0x92 => self.bitflag_u3_r8(BitflagOp::Res, 2, R8::D), // RES 2, D
            0x93 => self.bitflag_u3_r8(BitflagOp::Res, 2, R8::E), // RES 2, E
            0x94 => self.bitflag_u3_r8(BitflagOp::Res, 2, R8::H), // RES 2, H
            0x95 => self.bitflag_u3_r8(BitflagOp::Res, 2, R8::L), // RES 2, L
            0x96 => self.bitflag_u3_at_hl(BitflagOp::Res, 2),     // RES 2, [HL]
            0x97 => self.bitflag_u3_r8(BitflagOp::Res, 2, R8::A), // RES 2, A
            0x98 => self.bitflag_u3_r8(BitflagOp::Res, 3, R8::B), // RES 3, B
            0x99 => self.bitflag_u3_r8(BitflagOp::Res, 3, R8::C), // RES 3, C
            0x9A => self.bitflag_u3_r8(BitflagOp::Res, 3, R8::D), // RES 3, D
            0x9B => self.bitflag_u3_r8(BitflagOp::Res, 3, R8::E), // RES 3, E
            0x9C => self.bitflag_u3_r8(BitflagOp::Res, 3, R8::H), // RES 3, H
            0x9D => self.bitflag_u3_r8(BitflagOp::Res, 3, R8::L), // RES 3, L
            0x9E => self.bitflag_u3_at_hl(BitflagOp::Res, 3),     // RES 3, [HL]
            0x9F => self.bitflag_u3_r8(BitflagOp::Res, 3, R8::A), // RES 3, A

            0xA0 => self.bitflag_u3_r8(BitflagOp::Res, 4, R8::B), // RES 4, B
            0xA1 => self.bitflag_u3_r8(BitflagOp::Res, 4, R8::C), // RES 4, C
            0xA2 => self.bitflag_u3_r8(BitflagOp::Res, 4, R8::D), // RES 4, D
            0xA3 => self.bitflag_u3_r8(BitflagOp::Res, 4, R8::E), // RES 4, E
            0xA4 => self.bitflag_u3_r8(BitflagOp::Res, 4, R8::H), // RES 4, H
            0xA5 => self.bitflag_u3_r8(BitflagOp::Res, 4, R8::L), // RES 4, L
            0xA6 => self.bitflag_u3_at_hl(BitflagOp::Res, 4),     // RES 4, [HL]
            0xA7 => self.bitflag_u3_r8(BitflagOp::Res, 4, R8::A), // RES 4, A
            0xA8 => self.bitflag_u3_r8(BitflagOp::Res, 5, R8::B), // RES 5, B
            0xA9 => self.bitflag_u3_r8(BitflagOp::Res, 5, R8::C), // RES 5, C
            0xAA => self.bitflag_u3_r8(BitflagOp::Res, 5, R8::D), // RES 5, D
            0xAB => self.bitflag_u3_r8(BitflagOp::Res, 5, R8::E), // RES 5, E
            0xAC => self.bitflag_u3_r8(BitflagOp::Res, 5, R8::H), // RES 5, H
            0xAD => self.bitflag_u3_r8(BitflagOp::Res, 5, R8::L), // RES 5, L
            0xAE => self.bitflag_u3_at_hl(BitflagOp::Res, 5),     // RES 5, [HL]
            0xAF => self.bitflag_u3_r8(BitflagOp::Res, 5, R8::A), // RES 5, A

            0xB0 => self.bitflag_u3_r8(BitflagOp::Res, 6, R8::B), // RES 6, B
            0xB1 => self.bitflag_u3_r8(BitflagOp::Res, 6, R8::C), // RES 6, C
            0xB2 => self.bitflag_u3_r8(BitflagOp::Res, 6, R8::D), // RES 6, D
            0xB3 => self.bitflag_u3_r8(BitflagOp::Res, 6, R8::E), // RES 6, E
            0xB4 => self.bitflag_u3_r8(BitflagOp::Res, 6, R8::H), // RES 6, H
            0xB5 => self.bitflag_u3_r8(BitflagOp::Res, 6, R8::L), // RES 6, L
            0xB6 => self.bitflag_u3_at_hl(BitflagOp::Res, 6),     // RES 6, [HL]
            0xB7 => self.bitflag_u3_r8(BitflagOp::Res, 6, R8::A), // RES 6, A
            0xB8 => self.bitflag_u3_r8(BitflagOp::Res, 7, R8::B), // RES 7, B
            0xB9 => self.bitflag_u3_r8(BitflagOp::Res, 7, R8::C), // RES 7, C
            0xBA => self.bitflag_u3_r8(BitflagOp::Res, 7, R8::D), // RES 7, D
            0xBB => self.bitflag_u3_r8(BitflagOp::Res, 7, R8::E), // RES 7, E
            0xBC => self.bitflag_u3_r8(BitflagOp::Res, 7, R8::H), // RES 7, H
            0xBD => self.bitflag_u3_r8(BitflagOp::Res, 7, R8::L), // RES 7, L
            0xBE => self.bitflag_u3_at_hl(BitflagOp::Res, 7),     // RES 7, [HL]
            0xBF => self.bitflag_u3_r8(BitflagOp::Res, 7, R8::A), // RES 7, A

            0xC0 => self.bitflag_u3_r8(BitflagOp::Set, 0, R8::B), // SET 0, B
            0xC1 => self.bitflag_u3_r8(BitflagOp::Set, 0, R8::C), // SET 0, C
            0xC2 => self.bitflag_u3_r8(BitflagOp::Set, 0, R8::D), // SET 0, D
            0xC3 => self.bitflag_u3_r8(BitflagOp::Set, 0, R8::E), // SET 0, E
            0xC4 => self.bitflag_u3_r8(BitflagOp::Set, 0, R8::H), // SET 0, H
            0xC5 => self.bitflag_u3_r8(BitflagOp::Set, 0, R8::L), // SET 0, L
            0xC6 => self.bitflag_u3_at_hl(BitflagOp::Set, 0),     // SET 0, [HL]
            0xC7 => self.bitflag_u3_r8(BitflagOp::Set, 0, R8::A), // SET 0, A
            0xC8 => self.bitflag_u3_r8(BitflagOp::Set, 1, R8::B), // SET 1, B
            0xC9 => self.bitflag_u3_r8(BitflagOp::Set, 1, R8::C), // SET 1, C
            0xCA => self.bitflag_u3_r8(BitflagOp::Set, 1, R8::D), // SET 1, D
            0xCB => self.bitflag_u3_r8(BitflagOp::Set, 1, R8::E), // SET 1, E
            0xCC => self.bitflag_u3_r8(BitflagOp::Set, 1, R8::H), // SET 1, H
            0xCD => self.bitflag_u3_r8(BitflagOp::Set, 1, R8::L), // SET 1, L
            0xCE => self.bitflag_u3_at_hl(BitflagOp::Set, 1),     // SET 1, [HL]
            0xCF => self.bitflag_u3_r8(BitflagOp::Set, 1, R8::A), // SET 1, A

            0xD0 => self.bitflag_u3_r8(BitflagOp::Set, 2, R8::B), // SET 2, B
            0xD1 => self.bitflag_u3_r8(BitflagOp::Set, 2, R8::C), // SET 2, C
            0xD2 => self.bitflag_u3_r8(BitflagOp::Set, 2, R8::D), // SET 2, D
            0xD3 => self.bitflag_u3_r8(BitflagOp::Set, 2, R8::E), // SET 2, E
            0xD4 => self.bitflag_u3_r8(BitflagOp::Set, 2, R8::H), // SET 2, H
            0xD5 => self.bitflag_u3_r8(BitflagOp::Set, 2, R8::L), // SET 2, L
            0xD6 => self.bitflag_u3_at_hl(BitflagOp::Set, 2),     // SET 2, [HL]
            0xD7 => self.bitflag_u3_r8(BitflagOp::Set, 2, R8::A), // SET 2, A
            0xD8 => self.bitflag_u3_r8(BitflagOp::Set, 3, R8::B), // SET 3, B
            0xD9 => self.bitflag_u3_r8(BitflagOp::Set, 3, R8::C), // SET 3, C
            0xDA => self.bitflag_u3_r8(BitflagOp::Set, 3, R8::D), // SET 3, D
            0xDB => self.bitflag_u3_r8(BitflagOp::Set, 3, R8::E), // SET 3, E
            0xDC => self.bitflag_u3_r8(BitflagOp::Set, 3, R8::H), // SET 3, H
            0xDD => self.bitflag_u3_r8(BitflagOp::Set, 3, R8::L), // SET 3, L
            0xDE => self.bitflag_u3_at_hl(BitflagOp::Set, 3),     // SET 3, [HL]
            0xDF => self.bitflag_u3_r8(BitflagOp::Set, 3, R8::A), // SET 3, A

            0xE0 => self.bitflag_u3_r8(BitflagOp::Set, 4, R8::B), // SET 4, B
            0xE1 => self.bitflag_u3_r8(BitflagOp::Set, 4, R8::C), // SET 4, C
            0xE2 => self.bitflag_u3_r8(BitflagOp::Set, 4, R8::D), // SET 4, D
            0xE3 => self.bitflag_u3_r8(BitflagOp::Set, 4, R8::E), // SET 4, E
            0xE4 => self.bitflag_u3_r8(BitflagOp::Set, 4, R8::H), // SET 4, H
            0xE5 => self.bitflag_u3_r8(BitflagOp::Set, 4, R8::L), // SET 4, L
            0xE6 => self.bitflag_u3_at_hl(BitflagOp::Set, 4),     // SET 4, [HL]
            0xE7 => self.bitflag_u3_r8(BitflagOp::Set, 4, R8::A), // SET 4, A
            0xE8 => self.bitflag_u3_r8(BitflagOp::Set, 5, R8::B), // SET 4, B
            0xE9 => self.bitflag_u3_r8(BitflagOp::Set, 5, R8::C), // SET 5, C
            0xEA => self.bitflag_u3_r8(BitflagOp::Set, 5, R8::D), // SET 5, D
            0xEB => self.bitflag_u3_r8(BitflagOp::Set, 5, R8::E), // SET 5, E
            0xEC => self.bitflag_u3_r8(BitflagOp::Set, 5, R8::H), // SET 5, H
            0xED => self.bitflag_u3_r8(BitflagOp::Set, 5, R8::L), // SET 5, L
            0xEE => self.bitflag_u3_at_hl(BitflagOp::Set, 5),     // SET 5, [HL]
            0xEF => self.bitflag_u3_r8(BitflagOp::Set, 5, R8::A), // SET 5, A

            0xF0 => self.bitflag_u3_r8(BitflagOp::Set, 6, R8::B), // SET 6, B
            0xF1 => self.bitflag_u3_r8(BitflagOp::Set, 6, R8::C), // SET 6, C
            0xF2 => self.bitflag_u3_r8(BitflagOp::Set, 6, R8::D), // SET 6, D
            0xF3 => self.bitflag_u3_r8(BitflagOp::Set, 6, R8::E), // SET 6, E
            0xF4 => self.bitflag_u3_r8(BitflagOp::Set, 6, R8::H), // SET 6, H
            0xF5 => self.bitflag_u3_r8(BitflagOp::Set, 6, R8::L), // SET 6, L
            0xF6 => self.bitflag_u3_at_hl(BitflagOp::Set, 6),     // SET 6, [HL]
            0xF7 => self.bitflag_u3_r8(BitflagOp::Set, 6, R8::A), // SET 6, A
            0xF8 => self.bitflag_u3_r8(BitflagOp::Set, 7, R8::B), // SET 6, B
            0xF9 => self.bitflag_u3_r8(BitflagOp::Set, 7, R8::C), // SET 7, C
            0xFA => self.bitflag_u3_r8(BitflagOp::Set, 7, R8::D), // SET 7, D
            0xFB => self.bitflag_u3_r8(BitflagOp::Set, 7, R8::E), // SET 7, E
            0xFC => self.bitflag_u3_r8(BitflagOp::Set, 7, R8::H), // SET 7, H
            0xFD => self.bitflag_u3_r8(BitflagOp::Set, 7, R8::L), // SET 7, L
            0xFE => self.bitflag_u3_at_hl(BitflagOp::Set, 7),     // SET 7, [HL]
            0xFF => self.bitflag_u3_r8(BitflagOp::Set, 7, R8::A), // SET 7, A
        }
    }

    // Tons of instructions read or write at hl, so I extracted out the logic here
    fn read_at_hl(&self) -> u8 {
        let hl = self.reg.get16(R16::HL);
        self.mmu.read_byte(hl)
    }

    fn write_at_hl(&mut self, byte: u8) {
        let hl = self.reg.get16(R16::HL);
        self.mmu.write_byte(hl, byte);
    }

    // Misc instructions
    fn di(&mut self) {
        self.ime = false;
        self.ime_pending = false;
    }

    fn ei(&mut self) {
        self.ime_pending = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        todo!("Write CPU test cases");
    }
}