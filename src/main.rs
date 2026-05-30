#![allow(dead_code)]
#![allow(unused)]

mod cli;
mod cpu;
mod debugger;
mod mmu;
mod ppu;
mod ui;
mod util;

use cli::{Command, parse_cli_inputs};

use cpu::registers::R16;
use cpu::{Cpu, registers::R8};
use mmu::{Mmu, memmap::*};
use ppu::Ppu;
use sdl2::keyboard::Scancode;
use sdl2::sys::SDL_Scancode::{SDL_SCANCODE_C, SDL_SCANCODE_LCTRL};
use std::thread::sleep;
use std::time::{Duration, Instant};
use ui::Renderer;

use crate::debugger::{DebugState, debug_prompt};
use crate::ppu::FRAME_DOTS;
use crate::ui::Inputs;

const SYSTEM_CLOCK_FREQUENCY: f64 = (1 << 22) as f64; // Hz
const SYSTEM_CLOCK_PERIOD: f64 = 1.0 / SYSTEM_CLOCK_FREQUENCY; // Seconds
pub const M_CYCLE_DURATION: u32 = 4; // t-cycles
const GAMEBOY_FRAMERATE: f64 = 1.0 / 59.7275; // Seconds

fn main() {
    let input = parse_cli_inputs();
    match input {
        Command::Rom(path) => run_rom(&path, false),
        Command::Debug(path) => run_rom(&path, true),
        _ => (),
    }
}

fn run_rom(path: &str, mut debug_mode: bool) {
    println!("\nLoading rom at: \"{}\"", path);

    let (mut mmu, mut cpu, mut ppu) = create_gameboy_components();

    if !mmu.load_rom(path) {
        println!("Failed to load rom at \"{}\"", path);
        return;
    }

    initialize_memory(&mut mmu, &mut cpu);

    let (canvas, event_pump) = Renderer::init_window();
    let texture_creator = canvas.texture_creator();
    let mut renderer = Renderer::new(canvas, event_pump, &texture_creator);

    let framerate = Duration::from_secs_f64(GAMEBOY_FRAMERATE);
    let mut last_render_time = Instant::now();
    let mut time_elapsed: Duration;
    let mut frame_cycles: u32 = 0;
    let mut debug_state: DebugState = DebugState::None;

    while renderer.running {
        // TODO: for ppu object size bug, 8x16 mode should be set at pc = $026b

        debug_state = match debug_state {
            DebugState::Pause => debug_prompt(&cpu, &ppu, &mmu),
            DebugState::Step => DebugState::Step,
            DebugState::Continue => {
                tick_gameboy(&mut cpu, &mut ppu, &mut mmu, &mut renderer);
                DebugState::Continue
            }
            DebugState::None => {
                tick_gameboy(&mut cpu, &mut ppu, &mut mmu, &mut renderer);
                DebugState::None
            }
        };
        frame_cycles += 1;

        // TODO: Sleeping saves significant CPU power, but often causes oversleep
        if frame_cycles >= FRAME_DOTS {
            time_elapsed = last_render_time.elapsed();
            last_render_time += framerate;
            renderer.render_display(&ppu.display);
            renderer.process_inputs();
            update_joypad(&mut mmu, &renderer.inputs);
            if time_elapsed < framerate {
                sleep(framerate - time_elapsed);
            }
            frame_cycles = 0;

            // TODO: Entering and exiting debug mode causes the framerate timer to lag behind,
            //       and the game speed will temporarily increase to catch up
            if renderer.inputs.key_down[SDL_SCANCODE_LCTRL as usize]
                && renderer.inputs.keypress_unique[SDL_SCANCODE_C as usize]
            {
                debug_state = DebugState::Pause;
                println!("Debug mode");
            }
        }
    }
}

fn create_gameboy_components() -> (Mmu, Cpu, Ppu) {
    let mmu = Mmu::new();
    let cpu = Cpu::new();
    let ppu = Ppu::new();
    (mmu, cpu, ppu)
}

fn tick_gameboy(cpu: &mut Cpu, ppu: &mut Ppu, mmu: &mut Mmu, renderer: &mut Renderer) {
    cpu.tick(mmu);
    mmu.tick_timers();
    mmu.tick_dma();
    ppu.tick(mmu);
    if ppu.frame_complete {
        renderer.update_frame_from_display(&ppu.display, &mut ppu.frame_complete);
    }
}

/// While you technically can obtain a copy of the original gameboy bootrom online,
/// it's legally dubious. It's safer and easier for the user if the emulator just
/// replicates the post-boot state, rather than requiring them to source the bootrom.
/// [Pan Docs](https://gbdev.io/pandocs/Power_Up_Sequence.html?highlight=power%20up#power-up-sequence)
/// contains all of the necessary information to do this.
fn initialize_memory(mmu: &mut Mmu, cpu: &mut Cpu) {
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
    mmu.write_byte_override(NR_10_ADDR, 0x80);
    mmu.write_byte_override(NR_11_ADDR, 0xBF);
    mmu.write_byte_override(NR_12_ADDR, 0xF3);
    mmu.write_byte_override(NR_13_ADDR, 0xFF);
    mmu.write_byte_override(NR_14_ADDR, 0xBF);
    mmu.write_byte_override(NR_21_ADDR, 0x3F);
    mmu.write_byte_override(NR_22_ADDR, 0x00);
    mmu.write_byte_override(NR_23_ADDR, 0xFF);
    mmu.write_byte_override(NR_24_ADDR, 0xBF);
    mmu.write_byte_override(NR_30_ADDR, 0x7F);
    mmu.write_byte_override(NR_31_ADDR, 0xFF);
    mmu.write_byte_override(NR_32_ADDR, 0x9F);
    mmu.write_byte_override(NR_33_ADDR, 0xFF);
    mmu.write_byte_override(NR_34_ADDR, 0xBF);
    mmu.write_byte_override(NR_41_ADDR, 0xFF);
    mmu.write_byte_override(NR_42_ADDR, 0x00);
    mmu.write_byte_override(NR_43_ADDR, 0x00);
    mmu.write_byte_override(NR_44_ADDR, 0xBF);
    mmu.write_byte_override(NR_50_ADDR, 0x77);
    mmu.write_byte_override(NR_51_ADDR, 0xF3);
    mmu.write_byte_override(NR_52_ADDR, 0xF1);
    mmu.write_byte_override(LCDC_ADDR, 0x91);
    mmu.write_byte_override(STAT_ADDR, 0x85);
    mmu.write_byte_override(SCY_ADDR, 0x00);
    mmu.write_byte_override(SCX_ADDR, 0x00);
    mmu.write_byte_override(LY_ADDR, 0x00);
    mmu.write_byte_override(LYC_ADDR, 0x00);
    mmu.write_byte_override(DMA_ADDR, 0xFF);
    mmu.write_byte_override(BGP_ADDR, 0xFC);
    mmu.write_byte_override(OBP0_ADDR, 0x00);
    mmu.write_byte_override(OBP1_ADDR, 0x00);
    mmu.write_byte_override(WY_ADDR, 0x00);
    mmu.write_byte_override(WX_ADDR, 0x00);
    mmu.write_byte_override(IE_ADDR, 0x00);
}

fn update_joypad(mmu: &mut Mmu, inputs: &Inputs) {
    mmu.buttons.start = inputs.key_down[Scancode::Num1 as usize];
    mmu.buttons.select = inputs.key_down[Scancode::Num2 as usize];
    mmu.buttons.up = inputs.key_down[Scancode::W as usize];
    mmu.buttons.down = inputs.key_down[Scancode::S as usize];
    mmu.buttons.left = inputs.key_down[Scancode::A as usize];
    mmu.buttons.right = inputs.key_down[Scancode::D as usize];
    mmu.buttons.a = inputs.key_down[Scancode::Period as usize];
    mmu.buttons.b = inputs.key_down[Scancode::Comma as usize];
}
