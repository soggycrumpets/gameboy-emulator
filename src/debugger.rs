use std::{
    io::stdin,
    sync::mpsc::{Receiver, Sender},
    thread::current,
};

use crate::util::get_bit;

use super::*;

pub enum DebugState {
    Continue,
    Step,
    Pause,
    None,
}

pub fn run_debug_console(sender: Sender<String>) {
    let stdin = stdin();
    loop {
        let mut buffer = String::new();
        stdin
            .read_line(&mut buffer)
            .expect("failed to read from stdin");
        buffer = buffer.trim().to_string();
        sender.send(buffer).expect("Unable to send on channel");
    }
}

pub fn debug_prompt(cpu: &Cpu, ppu: &Ppu, mmu: &Mmu, rx: &Receiver<String>) -> DebugState {
    let command = if let Ok(message) = rx.try_recv() {
        message
    } else {
        return DebugState::Pause;
    };

    match command.as_str() {
        "step" | "s" => DebugState::Step,
        "continue" | "c" => DebugState::Continue,
        "print" | "p" => DebugState::Pause,
        "breakpoint" | "bp" => DebugState::Pause,
        "delete" | "d" => DebugState::Pause,
        "watchpoint" | "wp" => DebugState::Pause,
        "unwatch" | "uw" => DebugState::Pause,
        "list" | "l" => DebugState::Pause,
        "lcd" => {
            debug_lcd(ppu, mmu);
            DebugState::Pause
        }
        "dma" => DebugState::Pause,
        "registers" | "reg" => DebugState::Pause,
        _ => {
            println!("Unrecognized Command: \"{}\"", command);
            DebugState::Pause
        }
    }
}

pub fn debug_lcd(ppu: &Ppu, mmu: &Mmu) {
    // LCDC
    let lcdc = mmu.read_byte_override(LCDC_ADDR);
    let lcd_enabled = get_bit(lcdc, LCD_AND_PPU_ENABLE_BIT);
    let background_and_window = if get_bit(lcdc, BG_AND_WINDOW_ENABLE_BIT) {
        "Enabled"
    } else {
        "Disabled"
    };

    let objects = if get_bit(lcdc, OBJ_ENABLE_BIT) {
        "Enabled"
    } else {
        "Disabled"
    };

    let object_size = if get_bit(lcdc, OBJ_SIZE_BIT) {
        "8x16"
    } else {
        "8x8"
    };

    let background_tilemap = if get_bit(lcdc, BG_TILE_MAP_BIT) {
        format!("${:0x}", TILEMAP_1_ADDR)
    } else {
        format!("${:0x}", TILEMAP_0_ADDR)
    };

    let window = if get_bit(lcdc, WINDOW_ENABLE_BIT) {
        "Enabled"
    } else {
        "Disabled"
    };

    let window_tilemap = if get_bit(lcdc, WINDOW_TILE_MAP_BIT) {
        format!("${:04x}", TILEMAP_1_ADDR)
    } else {
        format!("${:04x}", TILEMAP_0_ADDR)
    };

    // STAT
    let stat = mmu.read_byte_override(STAT_ADDR);

    let current_mode = match ppu.get_mode(mmu) {
        ppu::PpuMode::HBlank => "Mode 0: HBlank",
        ppu::PpuMode::VBlank => "Mode 1: VBlank",
        ppu::PpuMode::OamScan => "Mode 2: OamScan",
        ppu::PpuMode::PixelDraw => "Mode 3: PixelDraw",
    };

    let lyc_flag = if get_bit(stat, LY_EQUALS_LYC_BIT) {
        "On"
    } else {
        "Off"
    };

    let hblank_interrupt = if get_bit(stat, MODE_0_INT_SELECT_BIT) {
        "Enabled"
    } else {
        "Disabled"
    };

    let vblank_interrupt = if get_bit(stat, MODE_1_INT_SELECT_BIT) {
        "Enabled"
    } else {
        "Disabled"
    };

    let oam_interrupt = if get_bit(stat, MODE_2_INT_SELECT_BIT) {
        "Enabled"
    } else {
        "Disabled"
    };

    let lyc_interrupt = if get_bit(stat, LYC_INT_SELECT_BIT) {
        "Enabled"
    } else {
        "Disabled"
    };

    // Other
    let ly = format!("{}", mmu.read_byte_override(LY_ADDR));
    let lyc = format!("{}", mmu.read_byte_override(LYC_ADDR));
    let window_position = format!(
        "({}, {})",
        mmu.read_byte_override(WX_ADDR),
        mmu.read_byte_override(WY_ADDR)
    );

    print!(
        "LCDC:
    LCD enabled: {}
    Background and Window: {}
    Objects: {}
    Object size: {}
    Background tilemap: {}
    Background and Window Tileset: {}
    Window: {}
    Window tilemap: {}

STAT:
    Current mode: {}
    LYC flag: {}
    H-Blank interrupt: {}
    V-Blank interrupt: {}
    OAM interrupt: {}
    LYC interrupt: {}

LY: {}
LYC: {}
Window position: {}
",
        // LCDC
        lcd_enabled,
        background_and_window,
        objects,
        object_size,
        background_tilemap,
        background_and_window,
        window,
        window_tilemap,
        // STAT
        current_mode,
        lyc_flag,
        hblank_interrupt,
        vblank_interrupt,
        oam_interrupt,
        lyc_interrupt,
        // Other
        ly,
        lyc,
        window_position,
    );
}

// fn parse_user_input(inputs: String) -> Command {
//     let mut args = inputs
//         .split_whitespace()
//         .map(|str| str.to_string())
//         .collect::<Vec<String>>();

//     args.reverse(); // Reverse args so popping from the back yields them in order

//     let arg = args.pop();
//     if arg.is_none() {
//         return Command::None;
//     }

//     // Map inputs to commands
//     match arg.unwrap().to_lowercase().as_str() {
//         "q" | "quit" => Command::Quit,
//         "n" | "step" => parse_step_arg(args),
//         "r" | "reg" => Command::PrintRegisters,
//         "m" | "vram" => Command::PrintVram,
//         "t" | "timer" => Command::PrintTimers,

//         _ => Command::None,
//     }
// }

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string()
}
