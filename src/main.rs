mod constants;
mod cpu;
mod mmu;
mod ppu;
mod util;
use std::{cell::RefCell, rc::Rc};

use constants::{
    PREFIXED_INSTRUCTION_T_CYCLE_TABLE, PROGRAM_START_ADDR, UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE,
};

use cpu::Cpu;
use mmu::Mmu;
use ppu::Ppu;

use cpu::registers::R16;

// Hardcoded for now
const BOOTROM_PATH: &str = "./roms/dmg_boot.gb";
const TEST_FULL_TEST_PATH: &str = "./test_roms/cpu_instrs.gb";

const GAME_PATH: &str = "./roms/tetris.gb";

const TEST_CPU_1_PATH: &str = "./test-roms/01-special.gb";
const TEST_CPU_2_PATH: &str = "./test-roms/02-interrupts.gb";
const TEST_CPU_3_PATH: &str = "./test-roms/03-op sp,hl.gb";
const TEST_CPU_4_PATH: &str = "./test-roms/04-op r,imm.gb";
const TEST_CPU_5_PATH: &str = "./test-roms/05-op rp.gb";
const TEST_CPU_6_PATH: &str = "./test-roms/06-ld r,r.gb";
const TEST_CPU_7_PATH: &str = "./test-roms/07-jr,jp,call,ret,rst.gb";
const TEST_CPU_8_PATH: &str = "./test-roms/08-misc instrs.gb";
const TEST_CPU_9_PATH: &str = "./test-roms/09-op r,r.gb";
const TEST_CPU_10_PATH: &str = "./test-roms/10-bit ops.gb";
const TEST_CPU_11_PATH: &str = "./test-roms/11-op a,(hl).gb";

const ROM_PATH: &str = GAME_PATH;
const TEST_ROM_PATH: &str = TEST_CPU_9_PATH;

fn main() {

    do_cpu_test();
    return;

    let mmu = Mmu::new();
    let mut cpu = Cpu::new(Rc::clone(&mmu));
    let mut ppu = Ppu::new(Rc::clone(&mmu));

    // TODO: Fix the boot sequence (not sure exactly how it should work yet)
    boot(&mut cpu);

    if !mmu.borrow_mut().load_rom(ROM_PATH) {
        println!("Failed to load \"{}\"", GAME_PATH);
        return;
    }

    loop {
        let pc = cpu.reg.get16(R16::PC);

        cpu.execute();
        let pc = cpu.reg.get16(R16::PC);
        // println!("{:4x}", pc);
    }
}

#[cfg(debug_assertions)]
fn do_cpu_test() {
    use constants::TOP_OF_STACK_ADDRESS;

    let mmu = Mmu::new();
    let mut cpu = Cpu::new(Rc::clone(&mmu));
    let mut ppu = Ppu::new(Rc::clone(&mmu));
    
    cpu.reg.set16(R16::PC, PROGRAM_START_ADDR);
    cpu.reg.set16(R16::SP, TOP_OF_STACK_ADDRESS);

    if !mmu.borrow_mut().load_test_rom(TEST_ROM_PATH) {
        panic!("Failed to load test rom at {}", TEST_ROM_PATH);
    }

    loop {
        let pc = cpu.reg.get16(R16::PC);
        cpu.execute();
        // println!("{:4x}", pc);
    } 
}

fn create_gameboy_components() -> (Cpu, Ppu) {
    let mmu = Mmu::new();
    let cpu = Cpu::new(Rc::clone(&mmu));
    let ppu = Ppu::new(Rc::clone(&mmu));
    (cpu, ppu)
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

fn test_rom() {
    let mmu = Mmu::new();
    let mut cpu = Cpu::new(Rc::clone(&mmu));

    if !cpu.mmu.borrow_mut().load_rom(ROM_PATH) {
        println!("Failed to CPU test rom");
        return;
    }

    loop {
        cpu.execute();
        let pc = cpu.reg.get16(R16::PC);
    }
}