use crate::{
    mmu::memmap::{DIV_ADDR, TAC_ADDR, TIMA_ADDR, TIMER_INTERRUPT_BIT, TMA_ADDR},
    util::{get_bit, set_bit},
};

use super::*;

// Timer frequencies are all derived by the 16-bit system clock, which increments once every
// m-cycle (every 4 t-cycles).

// TIMA increments when its corresponding bit in the system clock flips from 1 to 0.
// The corresponding bit is dictated by the lower two bits of TAC, which are mapped as follows.
// More information can be found at https://gbdev.io/pandocs/Timer_Obscure_Behaviour.html
const TAC_FREQ_1_SYSTEM_CLOCK_BIT: u8 = 3;
const TAC_FREQ_2_SYSTEM_CLOCK_BIT: u8 = 5;
const TAC_FREQ_3_SYSTEM_CLOCK_BIT: u8 = 7;
const TAC_FREQ_0_SYSTEM_CLOCK_BIT: u8 = 9;

const TAC_ENABLE_BIT: u8 = 2;
const T_CYCLES_PER_M_CYCLE: u16 = 4;

#[derive(Debug)]
pub struct Timers {
    pub system_clock: u16,
    pub system_clock_prev: u16,
    pub system_clock_counter: u16,
    tima_overflowed: bool,
}

impl Timers {
    pub fn new() -> Self {
        Timers {
            system_clock: 0,
            system_clock_prev: 0,
            system_clock_counter: 0,
            tima_overflowed: false,
        }
    }
}

impl Mmu {
    // One tick is 1 t-cycle
    pub fn tick_timers(&mut self) {
        self.timers.system_clock_counter += 1;
        // The system clock is a 16-bit number in t-cycles, but it is only incremented once every m-cycle.
        if self.timers.system_clock_counter == T_CYCLES_PER_M_CYCLE {
            self.timers.system_clock_counter = 0;
            self.timers.system_clock = self.timers.system_clock.wrapping_add(T_CYCLES_PER_M_CYCLE);
        }

        // DIV is the upper 8 bits of the system t cycle counter
        let div_value = (self.timers.system_clock >> 8) as u8;
        self.set_div_timer(div_value);

        // TIMA
        // todo! I think there's a problem in here. 3/4 Mooneye timer tests are failing by 1 timer count.
        let tac_enable = self.get_tac_enable();
        let tima_system_clock_bit = self.get_system_clock_bit_for_tima();
        let tima_bit_was_active =
            (self.timers.system_clock_prev & (1 << tima_system_clock_bit)) > 0;
        let tima_bit_is_active = (self.timers.system_clock & (1 << tima_system_clock_bit)) > 0;

        // TIMA's special overflow behavior occurs the cycle AFTER an overflow is detected.
        if self.timers.tima_overflowed {
            self.process_tima_overflow();
        }

        if tac_enable && tima_bit_was_active && !tima_bit_is_active {
            self.increment_tima();
        }

        // It's important to update the previous clockstate here, instead of at the beginning of the loop.
        // This is because the system clock can be reset if something writes to the div timer.
        // Therefore, the system clock state might be different by the next time this function is called.
        self.timers.system_clock_prev = self.timers.system_clock;
    }

    // The DIV address is special in that writes to it automatically set it to zero.
    // So we can't use the "write byte" function. We have to reach in directly.
    pub fn set_div_timer(&mut self, byte: u8) {
        let (_region, addr_mapped) = map_address(DIV_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = byte;
    }

    fn increment_tima(&mut self) {
        let mut tima_byte = self.read_byte(TIMA_ADDR);
        let overflowed;

        (tima_byte, overflowed) = tima_byte.overflowing_add(1);
        self.write_byte(TIMA_ADDR, tima_byte);

        if overflowed {
            self.timers.tima_overflowed = true;
        }
    }

    // Overflows send a timer interrupt
    // Instead of resetting to 0 on overflow, this timer is set to the value stored in TMA
    fn process_tima_overflow(&mut self) {
        self.timers.tima_overflowed = false;
        self.request_interrupt(TIMER_INTERRUPT_BIT);
        let tma_value = self.read_byte(TMA_ADDR);
        self.write_byte(TIMA_ADDR, tma_value);
    }

    // TAC enabled is the third bit from the right
    fn get_tac_enable(&self) -> bool {
        let byte = self.read_byte(TAC_ADDR);
        get_bit(byte, TAC_ENABLE_BIT)
    }

    // Clock select is the first two bits from the right
    fn get_system_clock_bit_for_tima(&self) -> u8 {
        let byte = self.read_byte(TAC_ADDR);
        let value = byte & 0b_0000_0011;
        match value {
            0 => TAC_FREQ_0_SYSTEM_CLOCK_BIT,
            3 => TAC_FREQ_3_SYSTEM_CLOCK_BIT,
            2 => TAC_FREQ_2_SYSTEM_CLOCK_BIT,
            1 => TAC_FREQ_1_SYSTEM_CLOCK_BIT,
            _ => unreachable!("Impossible value for TAC clock select"),
        }
    }

    // TODO: Writing to TAC may increase TIMA once! Perhaps this should be handled in the MMU.
    fn set_tac_enable(&mut self, set: bool) {
        let mut byte = self.read_byte(TAC_ADDR);
        set_bit(&mut byte, TAC_ENABLE_BIT, set);
        self.write_byte(TAC_ADDR, byte);
    }

    fn set_tac_clock_select(&mut self, value: u8) {
        let mut byte = self.read_byte(TAC_ADDR);
        let bit0 = get_bit(value, 0);
        let bit1 = get_bit(value, 1);
        set_bit(&mut byte, 0, bit0);
        set_bit(&mut byte, 1, bit1);
        self.write_byte(TAC_ADDR, byte);
    }
}

mod debug {
    use super::*;
    use crate::mmu::memmap::{DIV_ADDR, TIMA_ADDR};

    impl Mmu {
        pub fn print_timers(&mut self) {
            let div = self.read_byte(DIV_ADDR);
            let tima = self.read_byte(TIMA_ADDR);
            let tac_enable = self.get_tac_enable();
            let tima_bit = self.get_system_clock_bit_for_tima();
            println!("\nTimers:");
            print!(
                "\nSystem Clock: {:04x}, DIV: {:02x}, TIMA: {:02x}, TAC_ENABLE: {}, TIMA system clock bit: {:02}\n\n",
                self.timers.system_clock, div, tima, tac_enable, tima_bit
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TAC_CLOCK_1_T_CYCLE_PERIOD: u16 = 4 * 4;
    const TAC_CLOCK_2_T_CYCLE_PERIOD: u16 = 16 * 4;
    const TAC_CLOCK_3_T_CYCLE_PERIOD: u16 = 64 * 4;
    const TAC_CLOCK_0_T_CYCLE_PERIOD: u16 = 256 * 4;
    #[test]
    fn test_system_clock() {
        let mmu = Mmu::new();
        for _i in 0..3 {
            mmu.borrow_mut().tick_timers();
            assert_eq!(mmu.borrow().timers.system_clock, 0);
            assert_eq!(mmu.borrow().timers.system_clock_prev, 0);
        }
        mmu.borrow_mut().tick_timers();
        assert_eq!(mmu.borrow().timers.system_clock, 4);
        assert_eq!(mmu.borrow().timers.system_clock_prev, 0);
        mmu.borrow_mut().tick_timers();
        assert_eq!(mmu.borrow().timers.system_clock, 4);
        assert_eq!(mmu.borrow().timers.system_clock_prev, 4);
    }

    #[test]
    fn test_tima_clock_modes_() {
        test_clock_mode(1);
        test_clock_mode(2);
        test_clock_mode(3);
        test_clock_mode(0);
    }

    fn test_clock_mode(mode: u8) {
        let mmu = Mmu::new();
        mmu.borrow_mut().set_tac_enable(true);
        mmu.borrow_mut().set_tac_clock_select(mode);
        let period = match mode {
            1 => TAC_CLOCK_1_T_CYCLE_PERIOD,
            2 => TAC_CLOCK_2_T_CYCLE_PERIOD,
            3 => TAC_CLOCK_3_T_CYCLE_PERIOD,
            0 => TAC_CLOCK_0_T_CYCLE_PERIOD,
            _ => unreachable!(),
        };

        for _i in 0..period - 1 {
            mmu.borrow_mut().tick_timers();
            let tima = mmu.borrow().read_byte(TIMA_ADDR);
            assert_eq!(tima, 0);
        }
        mmu.borrow_mut().tick_timers();
        let tima = mmu.borrow().read_byte(TIMA_ADDR);
        assert_eq!(tima, 1);
    }
}
