use constants::{
    BOOTROM_START_ADDR, PREFIXED_INSTRUCTION_T_CYCLE_TABLE, PROGRAM_START_ADDR,
    UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE,
};
use cpu::Cpu;
use cpu::registers::R16;

mod constants;
mod cpu;
mod mmu;

const TEST_CPU_PATH: &str = "./roms/cpu_instrs.gb";
const GAME_PATH: &str = "./roms/tetris.gb";
const BOOTROM_PATH: &str = "./roms/dmg_boot.gb";

const ROM_PATH: &str = GAME_PATH;

fn main() {
    print_t_cycle_tables();
    return;

    let mut cpu = Cpu::new();
    // if !cpu.mmu.load_rom(BOOTROM_PATH, BOOTROM_START_ADDR) {
    //     println!("Failed to load bootrom");
    //     return;
    // }

    if !cpu.mmu.load_rom(ROM_PATH) {
        println!("Failed to load \"{}\"", GAME_PATH);
        return;
    }

    loop {
        cpu.execute();
        let pc = cpu.reg.get16(R16::PC);
        println!("{:4x}", pc);
    }
}

fn boot(cpu: &mut Cpu) -> bool {
    if !cpu.mmu.load_rom(ROM_PATH) {
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
    let mut cpu = Cpu::new();

    if !cpu.mmu.load_rom(ROM_PATH) {
        println!("Failed to CPU test rom");
        return;
    }

    loop {
        cpu.execute();
        let pc = cpu.reg.get16(R16::PC);
        println!("{:4x}", pc);
    }
}
