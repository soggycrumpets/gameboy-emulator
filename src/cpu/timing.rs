use crate::{constants::T_CYCLES_PER_M_CYCLE, mmu::memmap::{DIV_REGISTER_ADDR, TAC_REGISTER_ADDR, TIMA_REGISTER_ADDR, TMA_REGISTER_ADDR}};

use super::*;

// TODO: Implement obscure timer behaviors

// These contain the minimum number of tick cycles that each instruction can take to perform
pub const UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../../data/unprefixed_instruction_t_cycle_table.dat");
pub const PREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../../data/prefixed_instruction_t_cycle_table.dat");



const T_CYCLES_PER_DIVIDER_TIMER_INCREMENT: u32 = 256;


impl Cpu {
    pub fn update_timers(&mut self) {
        // DIV
        self.t_cycle_counter += self.instruction_tick_cycles as u32;
        if self.t_cycle_counter >= T_CYCLES_PER_DIVIDER_TIMER_INCREMENT {
            self.t_cycle_counter %= T_CYCLES_PER_DIVIDER_TIMER_INCREMENT;
            self.increment_div();
        }
    }

    fn increment_div(&self) {
        let mut timer = self.mmu.borrow().read_byte(DIV_REGISTER_ADDR);
        timer = timer.wrapping_add(1);
        self.mmu
            .borrow_mut()
            .write_byte(DIV_REGISTER_ADDR, timer);
    }

    fn increment_tima(&self) {
        let mut timer = self.mmu.borrow().read_byte(TIMA_REGISTER_ADDR);
        let overflow;
        (timer, overflow) = timer.overflowing_add(1);
        self.mmu
            .borrow_mut()
            .write_byte(TIMA_REGISTER_ADDR, timer);

        // When TIMA overflows, it resets to the value of TMA and a timer interrupt is requested
        if overflow {
            let reset_value = self.mmu.borrow().read_byte(TMA_REGISTER_ADDR);
            self.mmu
                .borrow_mut()
                .write_byte(TMA_REGISTER_ADDR, reset_value);
        }

        
    }

    // TAC enabled is the third bit from the right
    fn get_tac_enabled(&self) -> bool {
        let byte = self.mmu.borrow().read_byte(TAC_REGISTER_ADDR);
        let bit = byte & 0b_0000_0100;
        (bit >> 2) != 0
    }

    // Clock select is the first two bits from the right
    fn get_tac_frequency_in_t_cycles(&self) -> u32 {
        let byte = self.mmu.borrow().read_byte(TAC_REGISTER_ADDR);
        let value = byte & 0b_0000_0011;
        // The four values are mapped to frequencies (in t-cycles) as follows:
        match value {
            0b00 => 256 * T_CYCLES_PER_M_CYCLE,
            0b01 => 4 * T_CYCLES_PER_M_CYCLE,
            0b10 => 16 * T_CYCLES_PER_M_CYCLE,
            0b11 => 64 * T_CYCLES_PER_M_CYCLE,
            _ => unreachable!("Impossible value for TAC clock select"),
        }
    }

    // TODO: Writing to TAC may increase TIMA once! Perhaps this should be handled in the MMU.
    fn set_tac_enabled(&mut self, value: bool) {
        let mask = 0b_0000_0100;
        let bit = (value as u8) << 2;
        let mut byte = self.mmu.borrow().read_byte(TAC_REGISTER_ADDR);
        byte &= !mask;
        byte |= bit;
        self.mmu.borrow_mut().write_byte(TAC_REGISTER_ADDR, byte);
    }

    fn set_tac_frequency(&self, value: u8) {
        let mask = 0b_0000_0011;
        let bits = value & mask;
        let mut byte = self.mmu.borrow().read_byte(TAC_REGISTER_ADDR);
        byte &= !mask;
        byte |= bits;
        self.mmu.borrow_mut().write_byte(TAC_REGISTER_ADDR, byte);
    }
}

mod debug {
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
