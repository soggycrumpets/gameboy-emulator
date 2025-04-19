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
use mmu::{Mmu, memmap::*};
use ppu::Ppu;
use std::{
    cell::RefCell, process, rc::Rc, time::{Duration, Instant}
};
use ui::UserInterface;

use cpu::registers::R16;
const SYSTEM_CLOCK_FREQUENCY: f64 = (1 << 22) as f64; // Hz
const SYSTEM_CLOCK_PERIOD: f64 = 1.0 /SYSTEM_CLOCK_FREQUENCY; // Seconds

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
        process_inputs(&mut ui, &mmu);

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

            let pc = cpu.reg.get16(R16::PC);
        }
    }
}

fn create_gameboy_components() -> (Rc<RefCell<Mmu>>, Cpu, Ppu) {
    let mmu = Mmu::new();
    let cpu = Cpu::new(Rc::clone(&mmu));
    let ppu = Ppu::new(Rc::clone(&mmu));
    (mmu, cpu, ppu)
}

fn process_inputs(ui: &mut UserInterface, mmu: &Rc<RefCell<Mmu>>) {
    ui.process_inputs();
    let b = ui.inputs_down.w;
    let a_button = ui.inputs_down.a;
    let s_button = ui.inputs_down.s;
    let d_button = ui.inputs_down.d;

    let m_button = ui.inputs_down.m;
    let n_button = ui.inputs_down.n;


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

    // Hardware registers
    mmu.borrow_mut().write_byte_override(NR_10_ADDR, 0x80);
    mmu.borrow_mut().write_byte_override(NR_11_ADDR, 0xBF);
    mmu.borrow_mut().write_byte_override(NR_12_ADDR, 0xF3);
    mmu.borrow_mut().write_byte_override(NR_13_ADDR, 0xFF);
    mmu.borrow_mut().write_byte_override(NR_14_ADDR, 0xBF);
    mmu.borrow_mut().write_byte_override(NR_21_ADDR, 0x3F);
    mmu.borrow_mut().write_byte_override(NR_22_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(NR_23_ADDR, 0xFF);
    mmu.borrow_mut().write_byte_override(NR_24_ADDR, 0xBF);
    mmu.borrow_mut().write_byte_override(NR_30_ADDR, 0x7F);
    mmu.borrow_mut().write_byte_override(NR_31_ADDR, 0xFF);
    mmu.borrow_mut().write_byte_override(NR_32_ADDR, 0x9F);
    mmu.borrow_mut().write_byte_override(NR_33_ADDR, 0xFF);
    mmu.borrow_mut().write_byte_override(NR_34_ADDR, 0xBF);
    mmu.borrow_mut().write_byte_override(NR_41_ADDR, 0xFF);
    mmu.borrow_mut().write_byte_override(NR_42_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(NR_43_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(NR_44_ADDR, 0xBF);
    mmu.borrow_mut().write_byte_override(NR_50_ADDR, 0x77);
    mmu.borrow_mut().write_byte_override(NR_51_ADDR, 0xF3);
    mmu.borrow_mut().write_byte_override(NR_52_ADDR, 0xF1);
    mmu.borrow_mut().write_byte_override(LCDC_ADDR, 0x91);
    mmu.borrow_mut().write_byte_override(STAT_ADDR, 0x85);
    mmu.borrow_mut().write_byte_override(SCY_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(SCX_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(LY_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(LYC_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(DMA_ADDR, 0xFF);
    mmu.borrow_mut().write_byte_override(BGP_ADDR, 0xFC);
    mmu.borrow_mut().write_byte_override(OBP0_ADDR, 0x00); // Uninitialized
    mmu.borrow_mut().write_byte_override(OBP1_ADDR, 0x00); // Uninitialized
    mmu.borrow_mut().write_byte_override(WY_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(WX_ADDR, 0x00);
    mmu.borrow_mut().write_byte_override(IE_ADDR, 0x00);

}
