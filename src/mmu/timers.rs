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
const TIMA_WRITE_LOCK_T_CYCLES: u16 = 4;

#[derive(Debug)]
pub struct Timers {
    pub system_clock: u16,
    pub system_clock_prev: u16,
    pub system_clock_counter: u16,
    tima_overflowed: bool,
    tima_write_lock_counter: u16,
    tima_falling_edge_detected: bool,
}

impl Timers {
    pub fn new() -> Self {
        Timers {
            system_clock: 0,
            system_clock_prev: 0,
            system_clock_counter: 0,
            tima_overflowed: false,
            tima_write_lock_counter: 0,
            tima_falling_edge_detected: false,
        }
    }
}

impl Mmu {
    // One tick is 1 t-cycle
    pub fn tick_timers(&mut self) {
        self.timers.system_clock_counter += 1;
        self.timers.tima_write_lock_counter = self.timers.tima_write_lock_counter.saturating_sub(1);

        // The system clock is a 16-bit number in t-cycles, but it is only incremented once every m-cycle.
        if self.timers.system_clock_counter == T_CYCLES_PER_M_CYCLE {
            self.timers.system_clock_counter = 0;
            self.timers.system_clock = self.timers.system_clock.wrapping_add(T_CYCLES_PER_M_CYCLE);
        }

        // DIV is the upper 8 bits of the system t cycle counter
        let div_value = (self.timers.system_clock >> 8) as u8;
        self.bypass_write_byte_div(div_value);

        // TIMA
        let tac_enable = self.get_tac_enable();
        let tima_bit_is_active = self.get_tima_bit_state(false);
        let tima_bit_was_active = self.get_tima_bit_state(true);

        // TIMA's special overflow behavior occurs the cycle AFTER an overflow is detected.
        if self.timers.tima_overflowed {
            self.process_tima_overflow();
        }

        // TIMA increments while enabled, and on a falling clock edge. The second condition is
        // there because memory writes can trigger a clock edge detection that occurs elsewhere.
        if tac_enable && tima_bit_was_active && !tima_bit_is_active
            || self.timers.tima_falling_edge_detected
        {
            self.increment_tima();
            self.timers.tima_falling_edge_detected = false;
        }

        // It's important to update the previous clockstate here, instead of at the beginning of the loop.
        // This is because the system clock can be reset if something writes to the div timer.
        // Therefore, the system clock state might be different by the next time this function is called.
        self.timers.system_clock_prev = self.timers.system_clock;
    }

    fn increment_tima(&mut self) {
        let mut tima_byte = self.read_byte(TIMA_ADDR);
        let overflowed;

        (tima_byte, overflowed) = tima_byte.overflowing_add(1);

        self.bypass_write_byte_tima(tima_byte);

        if overflowed {
            self.timers.tima_overflowed = true;
        }
    }

    // Overflows send a timer interrupt
    // Instead of resetting to 0 on overflow, this timer is set to the value stored in TMA
    fn process_tima_overflow(&mut self) {
        self.timers.tima_overflowed = false;
        self.timers.tima_write_lock_counter = TIMA_WRITE_LOCK_T_CYCLES;

        self.request_interrupt(TIMER_INTERRUPT_BIT);
        let tma_value = self.read_byte(TMA_ADDR);
        self.bypass_write_byte_tima(tma_value);
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

    fn get_tima_bit_state(&self, prev_state: bool) -> bool {
        let initial_tima_system_clock_bit = self.get_system_clock_bit_for_tima();
        let system_clock = if prev_state {
            self.timers.system_clock_prev
        } else {
            self.timers.system_clock
        };
        (system_clock & (1 << initial_tima_system_clock_bit)) != 0
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

    // ----- Special-Case Memory Reads/Writes -----

    // DIV writes do not actually affect memory. Instead, they reset the system clock.
    // However, resetting the system clock can also increment TIMA if it unsets TIMA's
    // active bit!
    pub fn write_byte_div(&mut self) {
        let tima_bit_was_active = self.get_tima_bit_state(false);

        self.timers.system_clock = 0;

        let tima_bit_is_active = self.get_tima_bit_state(false);
        if tima_bit_was_active && !tima_bit_is_active {
            self.timers.tima_falling_edge_detected = true;
        }
    }

    // So we can't use the "write byte" function. We have to reach in directly.
    fn bypass_write_byte_div(&mut self, byte: u8) {
        let (_region, addr_mapped) = map_addr(DIV_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = byte;
    }

    // Writing to TIMA immediately after it has overflowed will cancel the overflow behavior.
    // Additionally, for the next machine cycle after TIMA overflows, it ignores writes!
    // This function is used by the general write_byte function as a wrapper around this weird behavior.
    pub fn write_byte_tima(&mut self, byte: u8) {
        let (_region, addr_mapped) = map_addr(TIMA_ADDR);
        let index = addr_mapped as usize;
        if self.timers.tima_write_lock_counter == 0 {
            self.io[index] = byte;
        }
        self.timers.tima_overflowed = false;
    }

    // While TIMA is write locked, some other timer functionality can still access it.
    // Namely, TIMA is still incremented, and TMA's value is still copied into TIMA while it is write protected.
    fn bypass_write_byte_tima(&mut self, byte: u8) {
        let (_region, addr_mapped) = map_addr(TIMA_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = byte;
    }

    // While TIMA is write locked, writing to TMA will also write to TIMA (bypassing the lock).
    // Additionally, writing to TMA can cause timer ticks in TIMA.

    pub fn write_byte_tma(&mut self, byte: u8) {
        let (_region, addr_mapped) = map_addr(TMA_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = byte;
        if self.timers.tima_write_lock_counter != 0 {
            self.bypass_write_byte_tima(byte);
        }
    }

    // If TIMA was tracking a set bit, but changes TAC change its tracking to an unset bit,
    // TIMA will see that is a falling edge and increment.
    // Additionally, disabling the timer while TIMA's selected bit was set will
    //  also trigger an increment
    pub fn write_byte_tac(&mut self, byte: u8) {
        let tima_bit_was_active = self.get_tima_bit_state(false);

        let (_region, addr_mapped) = map_addr(TAC_ADDR);
        let index = addr_mapped as usize;
        self.io[index] = byte;

        let tima_bit_is_active = self.get_tima_bit_state(false);
        let tima_is_enabled = self.get_tac_enable();

        if tima_bit_was_active && (!tima_bit_is_active || !tima_is_enabled) {
            self.timers.tima_falling_edge_detected = true;
        }
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

    #[test]
    fn test_div_write_causing_tima_increment() {
        {
            let mmu = Mmu::new();
            mmu.borrow_mut().timers.system_clock = 0xFFFF;
            mmu.borrow_mut().set_tac_enable(true);
            mmu.borrow_mut().set_tac_clock_select(0);

            mmu.borrow_mut().write_byte(DIV_ADDR, 0);

            assert_eq!(mmu.borrow_mut().timers.system_clock, 0);
            mmu.borrow_mut().tick_timers();
            let tima = mmu.borrow_mut().read_byte(TIMA_ADDR);
            assert_eq!(tima, 1);
        }
        {
            let mmu = Mmu::new();
            mmu.borrow_mut().timers.system_clock = 0xFFFF;
            mmu.borrow_mut().set_tac_enable(true);
            mmu.borrow_mut().set_tac_clock_select(0);

            mmu.borrow_mut().tick_timers();
            let tima = mmu.borrow_mut().read_byte(TIMA_ADDR);
            assert_eq!(tima, 0);
        }
    }
}
