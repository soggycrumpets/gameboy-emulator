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

use cpu::{registers::{Flag, R8}, Cpu};
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

fn main() {
    let input = parse_cli_inputs();
    match input {
        Command::Rom(path) => run_rom(&path),
        Command::Debug(path) => run_debug(&path),
    }
}

fn run_rom(path: &str) {
    println!("\nLoading rom at: \"{}\"", path);

    let (mut mmu, mut cpu, mut ppu) = create_gameboy_components();

    if !mmu.borrow_mut().load_rom(path) {
        println!("Failed to load rom at \"{}\"", path);
        return;
    }

    emulate_boot(&mmu, &mut cpu);

    let mut ui = UserInterface::new();

    while ui.running {
        cpu.execute();
        // ppu.draw();
        ui.process_inputs();
        // ui.render_display(&ppu.display);
    }
}

fn create_gameboy_components() -> (Rc<RefCell<Mmu>>, Cpu, Ppu) {
    let mmu = Mmu::new();
    let cpu = Cpu::new(Rc::clone(&mmu));
    let ppu = Ppu::new(Rc::clone(&mmu));
    (mmu, cpu, ppu)
}

// While you technically can obtain a copy of the original gameboy bootrom online,
// it's legally dubious. It's safer and easier for the user if the emulator just 
// reiplicates the post-boot state, rather than requiring them to source the bootrom.
// The pandocs contain good information about this (Section: 22. Power-Up Sequence)
// The bgb debugger also initializes its state this way, so I checked my values ag
fn emulate_boot(mmu: &Rc<RefCell<Mmu>>, cpu: &mut Cpu) {
    cpu.reg.set(R8::A, 0x01);
    // The H and C flags in the F register depend on the cartridge header checksum.
    // They are both true if checksum != 0x00, otherwise they are both false.
    // BGB initializes F to 0xB0 (checksum != 0x00), so I'll follow that example.
    cpu.reg.set(R8::F, 0xB0);
    cpu.reg.set(R8::B, 0x00);
    cpu.reg.set(R8::C, 0x13);
    cpu.reg.set(R8::D, 0x00);
    cpu.reg.set(R8::E, 0xD8);
    cpu.reg.set(R8::H, 0x01);
    cpu.reg.set(R8::L, 0x4D);
    cpu.reg.set16(R16::PC, 0x0100);
    cpu.reg.set16(R16::SP, 0xFFFE);
}
