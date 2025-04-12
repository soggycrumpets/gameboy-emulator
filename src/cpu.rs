use crate::memory::Mmu;
use crate::registers;
mod alu;
mod jumps;
mod load;
use registers::Flag;
use registers::Registers;
use registers::{R8, R16};

pub struct Cpu {
    reg: Registers,
    pub mmu: Mmu,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),
            mmu: Mmu::new(),
        }
    }

    pub fn fetch_byte(&mut self) -> u8 {
        let pc = self.reg.get16(R16::PC);
        let byte = self.mmu.readbyte(pc);

        let next_addr = pc + 1;
        self.reg.set16(R16::PC, next_addr);
        byte
    }

    pub fn fetch_word(&mut self) -> u16 {
        let pc = self.reg.get16(R16::PC);
        let word = self.mmu.readword(pc);

        let next_addr = pc + 2;
        self.reg.set16(R16::PC, next_addr);
        word
    }

    pub fn execute(&mut self) {
        let opcode = self.fetch_byte();
        println!("{:02x}", opcode);

        // Note: Every instruction that contains an n8, a8, or e8 will fetch a byte.
        // Every instruction that contains an n16 or a16 will fetch a word.

        match opcode {
            0x00 => (),                        // NOP
            0x01 => self.ld_r16_n16(R16::BC),  // LD BC, n16
            0x02 => self.ld_r16_a(R16::BC),    // LD BC, A
            0x03 => todo!("INC BC"),           // INC BC
            0x04 => self.inc_r8(R8::B),        // INC B
            0x05 => self.dec_r8(R8::B),        // DEC B
            0x06 => self.ld_r8_n8(R8::B),      // LD B, n8
            0x07 => todo!("RLCA"),             // RLCA
            0x08 => self.ld_at_n16_sp(),       // LD [n16], SP
            0x09 => self.add_hl_r16(R16::BC),  // ADD HL, BC
            0x0A => self.ld_a_at_r16(R16::BC), // LD A, [BC]
            0x0B => todo!("DEC BC"),           // DEC BC
            0x0C => self.inc_r8(R8::C),        // INC C
            0x0D => self.dec_r8(R8::C),        // DEC C
            0x0E => self.ld_r8_n8(R8::C),      // LD C, n8
            0x0F => todo!("RRCA"),             // RRCA

            0x10 => todo!("STOP n8"),          // STOP n8
            0x11 => self.ld_r16_n16(R16::DE),  // LD DE, n16
            0x12 => self.ld_r16_a(R16::DE),    // LD DE, A
            0x13 => todo!("INC DE"),           // INC DE
            0x14 => self.inc_r8(R8::D),        // INC D
            0x15 => self.dec_r8(R8::D),        // DEC D
            0x16 => self.ld_r8_n8(R8::D),      // LD D, n8
            0x17 => todo!("RLA"),              // RLA
            0x18 => self.jr_e8(),              // JR e8
            0x19 => todo!("ADD HL, DE"),       // ADD HL, DE
            0x1A => self.ld_a_at_r16(R16::DE), // LD A, [DE]
            0x1B => todo!("DEC DE"),           // DEC DE
            0x1C => self.inc_r8(R8::E),        // INC E
            0x1D => self.dec_r8(R8::E),        // DEC E
            0x1E => self.ld_r8_n8(R8::E),      // LD E, n8
            0x1F => todo!("RRA"),              // RRA

            0x20 => self.jr_cc_e8(Flag::Z, false), // JR NZ, e8
            0x21 => self.ld_r16_n16(R16::HL),      // LD HL, n16
            0x22 => self.ld_at_hli_a(),            // LD [HL+], A
            0x23 => todo!("INC HL"),               // INC HL
            0x24 => self.inc_r8(R8::H),            // INC H
            0x25 => self.dec_r8(R8::H),            // DEC H
            0x26 => self.ld_r8_n8(R8::H),          // LD H, n8
            0x27 => todo!("DAA"),                  // DAA
            0x28 => self.jr_cc_e8(Flag::Z, true),  // JR Z e8
            0x29 => todo!("ADD HL, HL"),           // ADD HL, HL
            0x2A => self.ld_a_at_hli(),            // LD A, [HL+]
            0x2B => todo!("DEC HL"),               // DEC HL
            0x2C => self.inc_r8(R8::L),            // INC L
            0x2D => self.dec_r8(R8::L),            // DEC L
            0x2E => self.ld_r8_n8(R8::L),          // LD L, n8
            0x2F => todo!("CPL"),                  // CPL

            0x30 => self.jr_cc_e8(Flag::C, false), // JR NC, e8
            0x31 => self.ld_r16_n16(R16::SP),      // LD SP, n16
            0x32 => self.ld_at_hld_a(),            // LD [HL-], A
            0x33 => todo!("INC SP"),               // INC SP
            0x34 => self.inc_at_hl(),              // INC [HL]
            0x35 => self.dec_at_hl(),              // DEC [HL]
            0x36 => self.ld_hl_n8(),               // LD [HL], n8
            0x37 => todo!("SCF"),                  // SCF
            0x38 => self.jr_cc_e8(Flag::C, true),  // JR C, e8
            0x39 => todo!("ADD HL, SP"),           // ADD HL, SP
            0x3A => self.ld_a_at_hld(),            // LD A, [HL-]
            0x3B => todo!("DEC SP"),               // DEC SP
            0x3C => self.inc_r8(R8::A),            // INC A
            0x3D => self.dec_r8(R8::A),            // DEC A
            0x3E => self.ld_r8_n8(R8::A),          // DH A, n8
            0x3F => todo!("CCF"),                  // CCF

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

            0x80 => self.add_a_r8(R8::B), // ADD A, B
            0x81 => self.add_a_r8(R8::C), // ADD A, C
            0x82 => self.add_a_r8(R8::D), // ADD A, D
            0x83 => self.add_a_r8(R8::E), // ADD A, E
            0x84 => self.add_a_r8(R8::H), // ADD A, H
            0x85 => self.add_a_r8(R8::L), // ADD A, L
            0x86 => self.add_a_at_hl(),   // ADD A, [HL]
            0x87 => self.add_a_r8(R8::A), // ADD A, A
            0x88 => self.adc_a_r8(R8::B), // ADC A, B
            0x89 => self.adc_a_r8(R8::C), // ADC A, C
            0x8A => self.adc_a_r8(R8::D), // ADC A, D
            0x8B => self.adc_a_r8(R8::E), // ADC A, E
            0x8C => self.adc_a_r8(R8::H), // ADC A, H
            0x8D => self.adc_a_r8(R8::L), // ADC A, L
            0x8E => self.adc_a_at_hl(),   // ADC A, [HL]
            0x8F => self.adc_a_r8(R8::A), // ADC A, A

            0x90 => self.sub_a_r8(R8::B), // SUB A, B
            0x91 => self.sub_a_r8(R8::C), // SUB A, C
            0x92 => self.sub_a_r8(R8::D), // SUB A, D
            0x93 => self.sub_a_r8(R8::E), // SUB A, E
            0x94 => self.sub_a_r8(R8::H), // SUB A, H
            0x95 => self.sub_a_r8(R8::L), // SUB A, L
            0x96 => self.sub_a_at_hl(),   // SUB A, [HL]
            0x97 => self.sub_a_r8(R8::A), // SUB A, A
            0x98 => self.sbc_a_r8(R8::B), // SBC A, B
            0x99 => self.sbc_a_r8(R8::C), // SBC A, C
            0x9A => self.sbc_a_r8(R8::D), // SBC A, D
            0x9B => self.sbc_a_r8(R8::E), // SBC A, E
            0x9C => self.sbc_a_r8(R8::H), // SBC A, H
            0x9D => self.sbc_a_r8(R8::L), // SBC A, L
            0x9E => self.sbc_a_at_hl(),   // SBC A, [HL]
            0x9F => self.sbc_a_r8(R8::A), // SBC A, A

            0xA0 => self.and_a_r8(R8::B), // AND A, B
            0xA1 => self.and_a_r8(R8::C), // AND A, C
            0xA2 => self.and_a_r8(R8::D), // AND A, D
            0xA3 => self.and_a_r8(R8::E), // AND A, E
            0xA4 => self.and_a_r8(R8::H), // AND A, H
            0xA5 => self.and_a_r8(R8::L), // AND A, L
            0xA6 => self.and_a_at_hl(),   // AND A, [HL]
            0xA7 => self.and_a_r8(R8::A), // AND A, A
            0xA8 => self.xor_a_r8(R8::B), // XOR A, B
            0xA9 => self.xor_a_r8(R8::C), // XOR A, C
            0xAA => self.xor_a_r8(R8::D), // XOR A, D
            0xAB => self.xor_a_r8(R8::E), // XOR A, E
            0xAC => self.xor_a_r8(R8::H), // XOR A, H
            0xAD => self.xor_a_r8(R8::L), // XOR A, L
            0xAE => self.xor_a_at_hl(),   // XOR A, [HL]
            0xAF => self.xor_a_r8(R8::A), // XOR A, A

            0xB0 => self.or_a_r8(R8::B), // OR A, B
            0xB1 => self.or_a_r8(R8::C), // OR A, C
            0xB2 => self.or_a_r8(R8::D), // OR A, D
            0xB3 => self.or_a_r8(R8::E), // OR A, E
            0xB4 => self.or_a_r8(R8::H), // OR A, H
            0xB5 => self.or_a_r8(R8::L), // OR A, L
            0xB6 => self.or_a_at_hl(),   // OR A, [HL]
            0xB7 => self.or_a_r8(R8::A), // OR A, A
            0xB8 => self.cp_a_r8(R8::B), // CP A, B
            0xB9 => self.cp_a_r8(R8::C), // CP A, C
            0xBA => self.cp_a_r8(R8::D), // CP A, D
            0xBB => self.cp_a_r8(R8::E), // CP A, E
            0xBC => self.cp_a_r8(R8::H), // CP A, H
            0xBD => self.cp_a_r8(R8::L), // CP A, L
            0xBE => self.cp_a_at_hl(),   // CP A, [HL]
            0xBF => self.cp_a_r8(R8::A), // CP A, A

            0xC0 => todo!("RET NZ"),               // RET NZ
            0xC1 => self.pop(R16::BC),             // POP BC
            0xC2 => self.jp_cc_a16(Flag::Z, false), // JP NZ
            0xC3 => self.jp_a16(),                 // JP a16
            0xC4 => todo!("CALL NZ, a16"),         // CALL NZ, a16
            0xC5 => self.push(R16::BC),            // PUSH BC
            0xC6 => self.add_a_n8(),               // ADD A, n8
            0xC7 => todo!("RST $00"),              // RST $00
            0xC8 => todo!("RET Z"),                // RET Z
            0xC9 => todo!("RET"),                  // RET
            0xCA => self.jp_cc_a16(Flag::Z, true), // JP Z, a16
            0xCB => todo!("PREFIX"),               // PREFIX
            0xCC => todo!("CALL Z, a16"),          // CALL Z, a16
            0xCD => todo!("CALL a16"),             // CALL a16
            0xCE => self.adc_a_n8(),               // ADC A, n8
            0xCF => todo!("RST $08"),              // RST $08

            0xD0 => todo!("RET NC"),                // RET NC
            0xD1 => self.pop(R16::DE),              // POP DE
            0xD2 => self.jp_cc_a16(Flag::C, false), // JP NC, a16
            0xD3 => unimplemented!(),               // ---
            0xD4 => todo!("CALL NC, a16"),          // CALL NC, a16
            0xD5 => self.push(R16::DE),             // PUSH DE
            0xD6 => self.sub_a_n8(),                // SUB A, n8
            0xD7 => todo!("RST $10"),               // RST $10
            0xD8 => todo!("RET C"),                 // RET C
            0xD9 => todo!("RETI"),                  // RETI
            0xDA => self.jp_cc_a16(Flag::C, true),  // JP C, a16
            0xDB => unimplemented!(),               // ---
            0xDC => todo!("CALL C, a16"),           // CALL C, a16
            0xDD => unimplemented!(),               // ---
            0xDE => self.sbc_a_n8(),                // SBC A, n8
            0xDF => todo!("RST $18"),               // RST $18

            0xE0 => self.ldh_at_a8_a(),  // LDH [a8], A
            0xE1 => self.pop(R16::HL),   // POP HL
            0xE2 => self.ldh_c_a(),      // LDH [C], A
            0xE3 => unimplemented!(),    // ---
            0xE4 => unimplemented!(),    // ---
            0xE5 => self.push(R16::HL),  // PUSH HL
            0xE6 => self.and_a_n8(),     // AND A, N8
            0xE7 => todo!("RST $20"),    // RST $20
            0xE8 => todo!("ADD SP, e8"), // ADD SP, e8
            0xE9 => self.jp_hl(),        // JP HL
            0xEA => self.ld_at_a16_a(),  // LD [a16], A
            0xEB => unimplemented!(),    // ---
            0xEC => unimplemented!(),    // ---
            0xED => unimplemented!(),    // ---
            0xEE => self.xor_a_n8(),     // XOR A, n8
            0xEF => todo!("RST $28"),    // RST $28

            0xF0 => self.ldh_a_a8(),    // LDH A, [a8]
            0xF1 => self.pop(R16::AF),  // POP AF
            0xF2 => self.ldh_a_at_c(),  // LDH A, [C]
            0xF3 => todo!("DI"),        // DI
            0xF4 => unimplemented!(),   // ---
            0xF5 => self.push(R16::AF), // PUSH AF
            0xF6 => self.or_a_n8(),     // OR A, n8
            0xF7 => todo!("RST $30"),   // RST $30
            0xF8 => self.ld_hl_sp_e8(), // LD HL, SP + e8
            0xF9 => self.ld_sp_hl(),    // LD SP, HL
            0xFA => self.ld_a_at_a16(), // LD A, [a16]
            0xFB => unimplemented!(),   // ---
            0xFC => unimplemented!(),   // ---
            0xFD => unimplemented!(),   // ---
            0xFE => self.cp_a_n8(),     // CP A, n8
            0xFF => todo!("RST $38"),   // RST $38
        }
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
