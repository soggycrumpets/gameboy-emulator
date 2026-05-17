use super::*;

mod alu;
mod bits;
mod instructions;
mod interrupts;
mod jumps;
mod loads;
pub mod registers;

use crate::mmu::Mmu;
use crate::mmu::memmap::{
    IE_ADDR, IF_ADDR, JOYPAD_INTERRUPT_BIT, JOYPAD_INTERRUPT_HANDLER_ADDR,
    SERIAL_INTERRUPT_BIT, SERIAL_INTERRUPT_HANDLER_ADDR, STAT_INTERRUPT_BIT,
    STAT_INTERRUPT_HANDLER_ADDR, TIMER_INTERRUPT_BIT, TIMER_INTERRUPT_HANDLER_ADDR,
    VBLANK_INTERRUPT_BIT, VBLANK_INTERRUPT_HANDLER_ADDR,
};

use crate::util::{get_bit, set_bit};

use alu::{AluBinary, AluUnary};
use bits::{BitflagOp, BitshiftOp};
use registers::{Flag, R8, R16, Registers};

pub const INTERRUPT_T_CYCLES: u8 = 20;

pub const UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../../data/unprefixed_instruction_t_cycle_table.dat");
pub const PREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../../data/prefixed_instruction_t_cycle_table.dat");

pub struct Cpu {
    pub reg: Registers,

    ime: bool,
    ime_pending: bool,
    halted: bool,
    halt_bug_active: bool,

    handling_interrupt: bool,
    current_interrupt_bit: u8,
    current_interrupt_handler_addr: u16,

    prev_instruction: u8,
    current_instruction: u8,
    prefixed_instruction_mode: bool,
    pub instruction_t_cycles_remaining: u8,
    instruction_m_cycles_remaining: u8,

    byte_buf: u8,
    word_buf_low: u8,
    word_buf_high: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg: Registers::new(),

            ime: true,
            ime_pending: false,
            halted: false,
            halt_bug_active: false,

            handling_interrupt: false,
            current_interrupt_bit: 0,
            current_interrupt_handler_addr: 0x0000,

            prev_instruction: 0,
            current_instruction: 0,
            prefixed_instruction_mode: false,
            instruction_t_cycles_remaining: 0,
            instruction_m_cycles_remaining: 0,

            byte_buf: 0x00,
            word_buf_high: 0x00,
            word_buf_low: 0x00,
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu) {
        // Update timings
        self.instruction_t_cycles_remaining = self.instruction_t_cycles_remaining.saturating_sub(1);
        self.instruction_m_cycles_remaining = self.instruction_t_cycles_remaining / 4;

        // One instruction per m-cycle
        if self.instruction_t_cycles_remaining.is_multiple_of(M_CYCLE_DURATION as u8) {
            self.step(mmu);
        }
    }

    fn step(&mut self, mmu: &mut Mmu) {
      

        self.update_interrupt_status(mmu);
        if self.ime_pending {
            self.ime = true;
            self.ime_pending = false;
        }

        if !self.halted && !self.handling_interrupt {
            if !self.prefixed_instruction_mode {
                self.execute(mmu);
            } else {
                self.execute_prefixed(mmu);
            }
        }

        if self.handling_interrupt {
            self.step_interrupt(mmu);
        }
    }

    fn fetch_byte(&mut self, mmu: &mut Mmu) -> u8 {
        let pc = self.reg.get16(R16::PC);
        let byte = mmu.read_byte(pc);

        let next_addr = if !self.halt_bug_active {
            pc + 1
        } else {
            self.halt_bug_active = false;
            pc
        };

        self.reg.set16(R16::PC, next_addr);
        byte
    }

    fn get_word_buf(&mut self) -> u16 {
        (self.word_buf_low as u16) | ((self.word_buf_high as u16) << 8)
    }

    fn fetch_word(&mut self, mmu: &mut Mmu) -> u16 {
        let pc = self.reg.get16(R16::PC);
        let word = mmu.read_word(pc);

        let next_addr = pc + 2;
        self.reg.set16(R16::PC, next_addr);

        word
    }
    // Tons of instructions read or write at hl, so I extracted out the logic here
    fn read_at_hl(&self, mmu: &mut Mmu) -> u8 {
        let hl = self.reg.get16(R16::HL);
        mmu.read_byte(hl)
    }

    fn write_at_hl(&mut self, byte: u8, mmu: &mut Mmu) {
        let hl = self.reg.get16(R16::HL);
        mmu.write_byte(hl, byte);
    }
   
}

pub mod debug {
    use super::*;
    pub fn print_t_cycle_tables() {
        println!("\nUnprefixed Instructions:\n");
        print_table(UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE);
        print!("\n\n");
        println!("Prefixed Instructions:\n");
        print_table(PREFIXED_INSTRUCTION_T_CYCLE_TABLE);
        print!("\n\n");

        fn print_table(table: &[u8]) {
            let mut counter = 0;
            for i in table {
                print!("{:02} ", i);
                counter += 1;

                if counter == 16 {
                    counter = 0;
                    println!();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
