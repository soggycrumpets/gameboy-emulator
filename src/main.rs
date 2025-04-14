mod constants;
mod util;
mod cpu;
mod mmu;
mod ppu;
use std::{cell::RefCell, rc::Rc};

use constants::{
    PREFIXED_INSTRUCTION_T_CYCLE_TABLE, PROGRAM_START_ADDR, UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE,
};

use cpu::Cpu;
use mmu::Mmu;
use ppu::Ppu;

use cpu::registers::R16;

// Hardcoded for now
const TEST_CPU_PATH: &str = "./roms/cpu_instrs.gb";
const GAME_PATH: &str = "./roms/tetris.gb";
const BOOTROM_PATH: &str = "./roms/dmg_boot.gb";

const ROM_PATH: &str = GAME_PATH;

fn main() {
    let mmu = Mmu::new();
    let mut cpu = Cpu::new(Rc::clone(&mmu));
    let mut ppu = Ppu::new(Rc::clone(&mmu));

    // TODO: Fix the boot sequence (not sure exactly how it should work yet)
    boot(&mut cpu);

    if !mmu.borrow_mut().load_rom(ROM_PATH) {
        println!("Failed to load \"{}\"", GAME_PATH);
        return;
    }

    loop {
        cpu.execute();
        let pc = cpu.reg.get16(R16::PC);
        // println!("{:4x}", pc);
    }
}

fn boot(cpu: &mut Cpu) -> bool {
    if !cpu.mmu.borrow_mut().load_rom(ROM_PATH) {
        println!("Failed to load \"{}\"", GAME_PATH);
        return false;
    }

    // Loop until the program counter reaches the end of the bootrom
    loop {
        let pc = cpu.reg.get16(R16::PC);
        if pc == PROGRAM_START_ADDR {
            break;
        }

        cpu.execute();
    }

    true
}

fn print_t_cycle_tables() {
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

fn test_rom() {
    let mmu = Mmu::new();
    let mut cpu = Cpu::new(Rc::clone(&mmu));

    if !cpu.mmu.borrow_mut().load_rom(ROM_PATH) {
        println!("Failed to CPU test rom");
        return;
    }

    loop {
        cpu.execute();
        let pc = cpu.reg.get16(R16::PC);
    }
}

enum Ahoy {
    There = 3,
    Matey = 7,
}

#[test]
fn sanity_check() {
    assert_eq!(Ahoy::There as u8, 3);
    assert_eq!(Ahoy::Matey as u8, 7);
}
