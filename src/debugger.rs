use crate::mmu::memmap::{PROGRAM_START_ADDR, TOP_OF_STACK_ADDRESS};

use super::*;

enum Command {
    Quit,
    Step(u64),
    PrintRegisters,
    PrintVram,
    PrintTimers,
    None,
}

pub fn debug_prompt(cpu: &Cpu, ppu: &Ppu, mmu: &Mmu, renderer: &Renderer) {
    println!("Debug mode");
    let command = get_user_input().to_lowercase();
    match command.as_str() {
        "step" | "s" => (),
        "continue" | "c" => (),
        "print" | "p" => (),
        "breakpoint" | "bp" => (),
        "delete" | "d" => (),
        "watchpoint" | "wp" => (),
        "unwatch" | "uw" => (),
        "list" | "l" => (),
        "lcd" => (),
        "dma" => (),
        "registers" | "reg" => (),
        _ => println!("Unrecognized Command: \"{}\"", command),
    }
}

pub fn debug_step() {

}

fn parse_user_input(inputs: String) -> Command {
    let mut args = inputs
        .split_whitespace()
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    args.reverse(); // Reverse args so popping from the back yields them in order

    let arg = args.pop();
    if arg.is_none() {
        return Command::None;
    }

    // Map inputs to commands
    match arg.unwrap().to_lowercase().as_str() {
        "q" | "quit" => Command::Quit,
        "n" | "step" => parse_step_arg(args),
        "r" | "reg" => Command::PrintRegisters,
        "m" | "vram" => Command::PrintVram,
        "t" | "timer" => Command::PrintTimers,

        _ => Command::None,
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string()
}

fn parse_step_arg(mut args: Vec<String>) -> Command {
    let arg = args.pop();
    if arg.is_none() {
        return Command::Step(1);
    }

    let steps: Option<u64> = arg.unwrap().parse().ok();

    if let Some(value) = steps {
        Command::Step(value)
    } else {
        Command::Step(1)
    }
}


