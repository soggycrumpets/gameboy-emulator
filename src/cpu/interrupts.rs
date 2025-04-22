use super::*;
/// the leftmost three bits of the IF register do not exist, and therefore always read high.
/// The leftmost three bits of the IE register DO exist, and can be written to.
/// When checking for pending interrupts, is it imperative that the upper three bits of the registers
/// be ignored. Otherwise, if any of IE's 3 upper bits happen to be set, it will wrongfully trigger interrupts.
const INTERRUPT_MASK: u8 = 0x1F;

/// In the case that an interrupt is cancelled by writing to IE mid-interrupt, the interrupt
/// return address is replaced with this.
const CANCELLED_INTERRUPT_RETURN_ADDR: u16 = 0x0000;

impl Cpu {
    pub fn update_interrupt_status(&mut self) {
        // Interrupts cannot be triggered in the middle of instructions.
        if self.instruction_m_cycles_remaining > 1 {
            return;
        }

        let ie_byte = self.read_byte(IE_ADDR);
        let if_byte = self.read_byte(IF_ADDR);
        let interrupts_are_pending = (ie_byte & if_byte & INTERRUPT_MASK) != 0;

        if !interrupts_are_pending {
            return;
        }

        self.halted = false;

        if !self.ime {
            return;
        }

        let vblank_interrupt = get_bit(if_byte, VBLANK_INTERRUPT_BIT);
        let stat_interrupt = get_bit(if_byte, STAT_INTERRUPT_BIT);
        let timer_interrupt = get_bit(if_byte, TIMER_INTERRUPT_BIT);
        let serial_interrupt = get_bit(if_byte, SERIAL_INTERRUPT_BIT);
        let joypad_interrupt = get_bit(if_byte, JOYPAD_INTERRUPT_BIT);

        let vblank_interrupt_enabled = get_bit(ie_byte, VBLANK_INTERRUPT_BIT);
        let stat_interrupt_enabled = get_bit(ie_byte, STAT_INTERRUPT_BIT);
        let timer_interrupt_enabled = get_bit(ie_byte, TIMER_INTERRUPT_BIT);
        let serial_interrupt_enabled = get_bit(ie_byte, SERIAL_INTERRUPT_BIT);
        let joypad_interrupt_enabled = get_bit(ie_byte, JOYPAD_INTERRUPT_BIT);

        // Interrupts are prioritized in order of their bit position (bit 0 first, bit 4 last)
        if vblank_interrupt && vblank_interrupt_enabled {
            self.start_interrupt(VBLANK_INTERRUPT_HANDLER_ADDR, VBLANK_INTERRUPT_BIT);
            // println!("VBLANK INTERRUPT");
        } else if stat_interrupt && stat_interrupt_enabled {
            self.start_interrupt(STAT_INTERRUPT_HANDLER_ADDR, STAT_INTERRUPT_BIT);
            // println!("STAT INTERRUPT");
        } else if timer_interrupt && timer_interrupt_enabled {
            self.start_interrupt(TIMER_INTERRUPT_HANDLER_ADDR, TIMER_INTERRUPT_BIT);
            // println!("TIMER INTERRUPT");
        } else if serial_interrupt && serial_interrupt_enabled {
            self.start_interrupt(SERIAL_INTERRUPT_HANDLER_ADDR, SERIAL_INTERRUPT_BIT);
            // println!("SERIAL INTERRUPT");
        } else if joypad_interrupt && joypad_interrupt_enabled {
            self.start_interrupt(JOYPAD_INTERRUPT_HANDLER_ADDR, JOYPAD_INTERRUPT_BIT);
        }
    }

    fn start_interrupt(&mut self, interrupt_handler_addr: u16, interrupt_bit: u8) {
        self.instruction_t_cycles_remaining = INTERRUPT_T_CYCLES;
        self.instruction_m_cycles_remaining =
            self.instruction_t_cycles_remaining / M_CYCLE_DURATION as u8;
        self.current_interrupt_handler_addr = interrupt_handler_addr;
        self.current_interrupt_bit = interrupt_bit;
        self.handling_interrupt = true;
    }

    /// Interrupts take 5 M-cycles, and are essentially an RST instruction that jumps to a
    /// special address, depending on which interrupt is being handled.
    /// Strange things can occur if the IE register is overwritten during the first byte push.
    /// I'm not managing to pass
    /// [Mooneye's test](https://github.com/Gekkio/mooneye-test-suite/blob/main/acceptance/interrupts/ie_push.s)
    /// for it, but but this behavior is obscure enough that it is unlikely to matter in almost any case.
    pub fn step_interrupt(&mut self) {
        match self.instruction_m_cycles_remaining {
            // NOP
            5 => (),
            // NOP
            4 => (),
            // Push PC high byte and update the status of the interrupt (cancelled?)
            3 => {
                let sp = self.reg.get16(R16::SP).wrapping_sub(1);
                self.reg.set16(R16::SP, sp);

                let high_byte = (self.reg.get16(R16::PC) >> 8) as u8;
                self.write_byte(sp, high_byte);

                let ie = self.read_byte(IE_ADDR);
                let enabled = get_bit(ie, self.current_interrupt_bit);

                self.ime = false;

                if !enabled {
                    self.current_interrupt_handler_addr = CANCELLED_INTERRUPT_RETURN_ADDR;
                } else {
                    let mut if_byte = self.read_byte(IF_ADDR);
                    set_bit(&mut if_byte, self.current_interrupt_bit, false);
                    self.write_byte(IF_ADDR, if_byte);
                }
            }
            // Push PC low byte
            2 => {
                // Decrement sp
                let sp = self.reg.get16(R16::SP).wrapping_sub(1);
                self.reg.set16(R16::SP, sp);

                let low_byte = self.reg.get16(R16::PC) as u8;
                self.write_byte(sp, low_byte);
            }
            // Jump to interrupt handler address
            1 => {
                self.jp_u16(self.current_interrupt_handler_addr);
                self.handling_interrupt = false;
            }
            _ => unreachable!(),
        }
    }

    // Interrupt Instructions
    pub fn di(&mut self) {
        self.ime = false;
        self.ime_pending = false;
    }

    pub fn ei(&mut self) {
        self.ime_pending = true;
    }

    // todo!
    pub fn stop(&mut self) {}

    pub fn halt(&mut self) {
        let ie_byte = self.read_byte(IE_ADDR);
        let if_byte = self.read_byte(IF_ADDR);
        let interrupts_are_pending = (ie_byte & if_byte & INTERRUPT_MASK) != 0;

        self.halted = !(self.ime && interrupts_are_pending);
        self.halt_bug_active = !self.ime && interrupts_are_pending;
    }
}
