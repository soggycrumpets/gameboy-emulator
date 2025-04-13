use constants::{BOOTROM_START_ADDR, PROGRAM_START_ADDR};
use cpu::Cpu;
use cpu::registers::R16;

mod constants;
mod cpu;
mod mmu;

const TEST_CPU_PATH: &str = "./roms/cpu_instrs.gb";
const GAME_PATH: &str = "./roms/tetris.gb";
const BOOTROM_PATH: &str = "./roms/dmg_boot.gb";

const ROM_PATH: &str = TEST_CPU_PATH;

fn main() {


    // test_rom();
    // return;

    let mut cpu = Cpu::new();
    if !cpu.mmu.load_rom(BOOTROM_PATH, BOOTROM_START_ADDR) {
        println!("Failed to load bootrom");
        return;
    }

    if !cpu.mmu.load_rom(ROM_PATH, PROGRAM_START_ADDR) {
        println!("Failed to load \"{}\"", GAME_PATH);
        return;
    }

    loop {
        cpu.execute();
        // let pc = cpu.reg.get16(R16::PC);
        // println!("{:4x}", pc);
    }
}


fn test_rom() {
    let mut cpu = Cpu::new();

    if !cpu.mmu.load_rom(ROM_PATH, BOOTROM_START_ADDR) {
        println!("Failed to CPU test rom");
        return;
    }

    loop {
        cpu.execute();
        let pc = cpu.reg.get16(R16::PC);
        println!("{:4x}", pc);
    }
}