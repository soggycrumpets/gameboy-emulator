use crate::{
    constants::T_CYCLES_PER_M_CYCLE,
    mmu::memmap::{DIV_ADDR, IF_ADDR, TAC_ADDR, TIMA_ADDR, TMA_ADDR},
    util::{get_bit, set_bit},
};

use super::*;

// TODO: Implement obscure timer behaviors

// These contain the minimum number of tick cycles that each instruction can take to perform
pub const UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../../data/unprefixed_instruction_t_cycle_table.dat");
pub const PREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../../data/prefixed_instruction_t_cycle_table.dat");

const T_CYCLES_PER_DIV_INCREMENT: u16 = 256;

const IF_TIMER_BIT: u8 = 2;
const TAC_ENABLE_BIT: u8 = 3;

impl Cpu {
    pub fn update_timers(&mut self) {
        let prev_t_cycles = self.t_cycle_counter;
        self.t_cycle_counter += self.instruction_t_cycles as u64;
        let t_cycles = self.t_cycle_counter;

        // DIV
        if (t_cycles / T_CYCLES_PER_DIV_INCREMENT as u64)
            > (prev_t_cycles / T_CYCLES_PER_DIV_INCREMENT as u64)
        {
            self.increment_div();
        }

        // TIMA
        let tac_enable = self.get_tac_enable();
        let t_cycles_per_tima_increment = self.get_tac_frequency_in_t_cycles();
        if tac_enable
            && ((t_cycles / t_cycles_per_tima_increment as u64)
                > (prev_t_cycles / t_cycles_per_tima_increment as u64))
        {
            self.increment_tima();
        }
    }

    fn increment_div(&self) {
        let mut timer = self.read_byte(DIV_ADDR);
        timer = timer.wrapping_add(1);
        self.write_byte(DIV_ADDR, timer);
    }

    fn increment_tima(&self) {
        let mut timer = self.read_byte(TIMA_ADDR);
        let overflow;
        (timer, overflow) = timer.overflowing_add(1);
        self.write_byte(TIMA_ADDR, timer);

        // When TIMA overflows, it resets to the value of TMA and a timer interrupt is requested
        if overflow {
            let reset_value = self.read_byte(TMA_ADDR);
            self.write_byte(TMA_ADDR, reset_value);
            self.set_timer_interrupt(true);
        }
    }

    // TAC enabled is the third bit from the right
    fn get_tac_enable(&self) -> bool {
        let byte = self.read_byte(TAC_ADDR);
        get_bit(byte, IF_TIMER_BIT)
    }

    // Clock select is the first two bits from the right
    fn get_tac_frequency_in_t_cycles(&self) -> u32 {
        let byte = self.read_byte(TAC_ADDR);
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
    fn set_tac_enabled(&mut self, set: bool) {
        let mut byte = self.read_byte(TAC_ADDR);
        set_bit(&mut byte, TAC_ENABLE_BIT, set);
        self.write_byte(TAC_ADDR, byte);
    }

    fn set_tac_frequency(&self, value: u8) {
        let mut byte = self.read_byte(TAC_ADDR);
        let bit1 = get_bit(value, 1);
        let bit2 = get_bit(value, 2);
        set_bit(&mut byte, 1, bit1);
        set_bit(&mut byte, 2, bit2);
        self.write_byte(TAC_ADDR, byte);
    }

    fn set_timer_interrupt(&self, set: bool) {
        let mut byte = self.read_byte(IF_ADDR);
        set_bit(&mut byte, IF_TIMER_BIT, set);
        self.write_byte(IF_ADDR, byte);
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
