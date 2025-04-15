use std::{fmt::Debug, num::ParseIntError};

use super::*;
use crate::constants::{
    PREFIXED_INSTRUCTION_T_CYCLE_TABLE, TOP_OF_STACK_ADDRESS, UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE,
};

enum DebugCommand {
    Quit,
    Step(u32),
    PrintRegisters,
    PrintVram,
    None,
}

pub fn run_debug(path: &str) {
    println!("\nLoading rom at: \"{}\"", path);

    let (mmu, mut cpu, mut ppu) = create_gameboy_components();

    cpu.reg.set16(R16::PC, PROGRAM_START_ADDR);
    cpu.reg.set16(R16::SP, TOP_OF_STACK_ADDRESS);

    if !mmu.borrow_mut().load_rom(path) {
        println!("Failed to load \"{}\"", path);
        return;
    }

    let mut ui = UserInterface::new();
    let mut running = true;

    while running {
        ui.process_inputs();
        ui.render_display(&ppu.display);

        let input = get_user_input();
        let command = parse_user_input(input);

        match command {
            DebugCommand::Quit => running = false,
            DebugCommand::Step(count) => step_gameboy(count, &mut cpu, &mut ppu),
            DebugCommand::PrintVram => mmu.borrow().print_vram(),
            DebugCommand::PrintRegisters => cpu.reg.print(),
            DebugCommand::None => println!("Unrecognized Command"),
        }
    }
}

fn parse_user_input(inputs: String) -> DebugCommand {
    let mut args = inputs
        .split_whitespace()
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    args.reverse(); // Reverse args so popping from the back yields them in order

    let arg = args.pop();
    if arg.is_none() {
        return DebugCommand::None;
    }

    // Map inputs to commands
    match arg.unwrap().to_lowercase().as_str() {
        "q" | "quit" => DebugCommand::Quit,
        "n" | "step" => parse_step_arg(args),
        "r" | "printreg" => DebugCommand::PrintRegisters,
        "m" | "printvram" => DebugCommand::PrintVram,

        _ => DebugCommand::None,
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string()
}

fn parse_step_arg(mut args: Vec<String>) -> DebugCommand {
    let arg = args.pop();
    if arg.is_none() {
        return DebugCommand::Step(1);
    }

    let steps: Option<u32> = arg.unwrap().parse().ok();

    if let Some(value) = steps {
        DebugCommand::Step(value)
    } else {
        DebugCommand::Step(1)
    }
}

fn step_gameboy(count: u32, cpu: &mut Cpu, ppu: &mut Ppu) {
    for _i in 0..count {
        cpu.execute();
        ppu.draw();
    }
    println!("Stepped {} cycle(s)", count);
}

pub fn print_t_cycle_tables() {
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
