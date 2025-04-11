use constants::{BOOTROM_START_ADDR, PROGRAM_START_ADDR};
use cpu::CPU;

mod constants;
mod cpu;
mod memory;
mod registers;

const GAME_PATH: &str = "./roms/tetris.gb";
const BOOTROM_PATH: &str = "./roms/dmg_boot.gb";

// struct Gameboy {
//     cpu: CPU,
// }

// impl Gameboy {
//     fn new() -> Self {
//         Gameboy {
//             cpu: CPU::new(),
//             ram: RAM::new(),
//         }
//     }
// }

fn main() {
    let mut cpu = CPU::new();
    if !cpu.mmu.load_rom(BOOTROM_PATH, BOOTROM_START_ADDR) {
        println!("Failed to load bootrom"); 
        return;
    }

    if !cpu.mmu.load_rom(GAME_PATH, PROGRAM_START_ADDR) {
        println!("Failed to load \"{}\"", GAME_PATH);
        return;
    }

    loop {
        cpu.execute();
    }
    // for byte in &gameboy.ram.memory {
    // print!("{:x}", byte);
    // }
}
