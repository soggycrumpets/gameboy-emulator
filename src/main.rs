mod cli;
mod constants;
mod cpu;
mod mmu;
mod ppu;
mod ui;
mod util;

use cli::{Command, check_cli_inputs, parse_cli_inputs};
use constants::{
    PREFIXED_INSTRUCTION_T_CYCLE_TABLE, PROGRAM_START_ADDR, UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE,
};

use cpu::Cpu;
use mmu::Mmu;
use ppu::Ppu;
use ui::UserInterface;
use std::{cell::RefCell, rc::Rc};

use cpu::registers::R16;

// Hardcoded for now
const BOOTROM_PATH: &str = "./roms/dmg_boot.gb";
const TEST_FULL_TEST_PATH: &str = "./test_roms/cpu_instrs.gb";

const GAME_PATH: &str = "./roms/tetris.gb";
const ROM_PATH: &str = GAME_PATH;

fn run_gameboy(mut cpu: Cpu, mut ppu: Ppu) {
    let mut ui = UserInterface::new();

    while ui.running {
        cpu.execute();
        ppu.draw();
        ui.process_inputs();
        ui.render_display(&ppu.display);
        // print!("{:?}", ppu.screen);
    }
}

fn main() {
    if check_cli_inputs() {
        return;
    }


    let (mmu, mut cpu, ppu) = create_gameboy_components();

    // TODO: Fix the boot sequence (not sure exactly how it should work yet)
    // boot(&mut cpu);

    if !mmu.borrow_mut().load_rom(ROM_PATH) {
        println!("Failed to load \"{}\"", ROM_PATH);
        return;
    }

    cpu.reg.set16(R16::PC, 0x0100);
    run_gameboy(cpu, ppu);
}

#[cfg(debug_assertions)]
fn do_cpu_test(path: &str) {
    use constants::TOP_OF_STACK_ADDRESS;

    let mmu = Mmu::new();
    let mut cpu = Cpu::new(Rc::clone(&mmu));
    let mut ppu = Ppu::new(Rc::clone(&mmu));

    cpu.reg.set16(R16::PC, PROGRAM_START_ADDR);
    cpu.reg.set16(R16::SP, TOP_OF_STACK_ADDRESS);

    if !mmu.borrow_mut().load_test_rom(path) {
        panic!("Failed to load test rom at {}", path);
    }

    run_gameboy(cpu, ppu);
}

fn create_gameboy_components() -> (Rc<RefCell<Mmu>>, Cpu, Ppu) {
    let mmu = Mmu::new();
    let cpu = Cpu::new(Rc::clone(&mmu));
    let ppu = Ppu::new(Rc::clone(&mmu));
    (mmu, cpu, ppu)
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
