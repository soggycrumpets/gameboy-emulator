use super::*;

pub enum Interrupt {}

impl Cpu {
    pub fn handle_interrupts(&mut self) -> bool {
        if self.handling_interrupt {
            self.step_interrupt();
            return true;
        }

        // Interrupts should not be updated mid-instruction
        if self.instruction_t_cycles_remaining != 0 {
            return false;
        }

        let ie_byte = self.read_byte(IE_ADDR);
        let if_byte = self.read_byte(IF_ADDR);
        let interrupts_are_pending = (ie_byte & if_byte) != 0;

        if !interrupts_are_pending {
            return false;
        }

        if !self.ime {
            self.halted = false;
            return false;
        }

        self.halted = false;

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

        true
    }

    fn start_interrupt(&mut self, interrupt_handler_addr: u16, interrupt_bit: u8) {
        // Record that the interrupt has been handled
        // let mut if_byte = self.read_byte(IF_ADDR);
        // set_bit(&mut if_byte, interrupt_bit, false);
        // self.write_byte(IF_ADDR, if_byte);
        // self.ime = false;

        self.instruction_t_cycles_remaining = INTERRUPT_T_CYCLES;
        self.instruction_m_cycles_remaining =
            self.instruction_t_cycles_remaining / M_CYCLE_DURATION as u8;
        self.current_interrupt_handler_addr = interrupt_handler_addr;
        self.current_interrupt_bit = interrupt_bit;
        self.handling_interrupt = true;
        self.step_interrupt();
    }

    fn step_interrupt(&mut self) {
        match (self.instruction_m_cycles_remaining) {
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

                //
                let ie = self.read_byte(IE_ADDR);
                let enabled = get_bit(ie, self.current_interrupt_bit);
                if !enabled {
                    self.current_interrupt_handler_addr = 0x0000;
                } else {
                    let mut if_byte = self.read_byte(IF_ADDR);
                    set_bit(&mut if_byte, self.current_interrupt_bit, false);
                    self.write_byte(IF_ADDR, if_byte);
                    self.ime = false;
                }
                println!("3");
            }
            // Push PC low byte
            2 => {
                // Decrement sp
                let sp = self.reg.get16(R16::SP).wrapping_sub(1);
                self.reg.set16(R16::SP, sp);

                let low_byte = self.reg.get16(R16::PC) as u8;
                self.write_byte(sp, low_byte);
                println!("2");
            }
            // Jump to interrupt handler address
            1 => {
                self.jp_u16(self.current_interrupt_handler_addr);
                self.handling_interrupt = false;
                println!("1");
            }
            _ => unreachable!("{}", self.interrupt_t_cycles_remaining),
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
        let interrupts_are_pending = (ie_byte & if_byte) != 0;

        if !self.ime && interrupts_are_pending {
            self.halt_bug_active = true;
        } else {
            self.halted = true;
        }
    }
}
