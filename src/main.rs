mod cli;
mod constants;
mod cpu;
mod debugger;
mod mmu;
mod ppu;
mod ui;
mod util;

use cli::{Command, parse_cli_inputs};

use cpu::{Cpu, registers::R8};
use debugger::run_debug;
use mmu::{memmap::{IF_ADDR, JOYPAD_INPUT_ADDR, LYC_ADDR, SCY_ADDR}, Mmu};
use ppu::Ppu;
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};
use ui::UserInterface;

use cpu::registers::R16;
const SYSTEM_CLOCK_FREQUENCY: f64 = (1 << 22) as f64; // Hz
const SYSTEM_CLOCK_PERIOD: f64 = 1.0 / SYSTEM_CLOCK_FREQUENCY; // Seconds

fn main() {
    let input = parse_cli_inputs();
    match input {
        Command::Rom(path) => run_rom(&path),
        Command::Debug(path) => run_debug(&path),
    }
}

fn run_rom(path: &str) {
    println!("\nLoading rom at: \"{}\"", path);

    let (mmu, mut cpu, mut ppu) = create_gameboy_components();

    if !mmu.borrow_mut().load_rom(path) {
        println!("Failed to load rom at \"{}\"", path);
        return;
    }

    emulate_boot(&mmu, &mut cpu);

    let mut ui = UserInterface::new();

    let render_timer_period = Duration::from_secs_f64(1.0 / 60.0);
    let mut last_render_time = Instant::now();

    // todo! This loop munches up CPU
    // todo! The only timer this should need is the global clock,
    // One loop represents one t-cycle
    while ui.running {
        ui.process_inputs();

        cpu.tick();
        

        mmu.borrow_mut().tick_timers();
        ppu.tick();
        // todo!
        // The ppu should eventually draw a little bit at a time.
        // For now, just draw everything at once at 60fps
        if last_render_time.elapsed() >= render_timer_period {
            ppu.splat_tiles();
            ui.render_display(&ppu.display);
            last_render_time = Instant::now();

            let ly = mmu.borrow().read_byte(LYC_ADDR);
            let sc = mmu.borrow().read_byte(SCY_ADDR);
            // println!("{:02x} : {:02x}", ly, sc);
        }
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
// replicates the post-boot state, rather than requiring them to source the bootrom.
// The pandocs contain good information about this (Section: 22. Power-Up Sequence)
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

    // Joypad bits read high by default, 0 when pressed
    mmu.borrow_mut().write_byte(JOYPAD_INPUT_ADDR, 0xFF);
    // As far as I can tell from online, the upper 3 bits of this register don't actually exist.
    // And because of this, they always read high. So I will have the memory reflect this.
    mmu.borrow_mut().write_byte(IF_ADDR, 0b_1110_0000);
}
