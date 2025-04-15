mod cli;
mod constants;
mod cpu;
mod debugger;
mod mmu;
mod ppu;
mod ui;
mod util;

use cli::{Command, parse_cli_inputs};
use constants::PROGRAM_START_ADDR;

use cpu::Cpu;
use debugger::run_debug;
use mmu::Mmu;
use ppu::Ppu;
use std::{cell::RefCell, rc::Rc};
use ui::UserInterface;

use cpu::registers::R16;

// Hardcoded for now
const BOOTROM_PATH: &str = "./roms/dmg_boot.gb";
const TEST_FULL_TEST_PATH: &str = "./test_roms/cpu_instrs.gb";

const GAME_PATH: &str = "./roms/tetris.gb";
const ROM_PATH: &str = GAME_PATH;

fn run_rom(path: &str) {

    println!("\nLoading rom at: \"{}\"", path);

    use constants::TOP_OF_STACK_ADDRESS;

    let (mmu, mut cpu, mut ppu) = create_gameboy_components();

    cpu.reg.set16(R16::PC, PROGRAM_START_ADDR);
    cpu.reg.set16(R16::SP, TOP_OF_STACK_ADDRESS);

    if !mmu.borrow_mut().load_rom(path) {
        println!("Failed to load \"{}\"", path);
        return;
    }

    let mut ui = UserInterface::new();

    while ui.running {
        cpu.execute();
        ppu.draw();
        ui.process_inputs();
        ui.render_display(&ppu.display);
    }
}

fn main() {
    let input = parse_cli_inputs();
    match input {
        Command::Rom(path) => run_rom(&path),
        Command::Debug(path) => run_debug(&path),
    }
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