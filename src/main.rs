use cpu::CPU;

mod cpu;
mod memory;
mod registers;
mod constants;

const ROM_PATH: &str = "./roms/tetris.gb";

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
    if !cpu.mmu.load_rom(ROM_PATH) {
        println!("Failed to load \"{}\"", ROM_PATH);
        return;
    }

    loop {
        cpu.execute();
    }
    // for byte in &gameboy.ram.memory {
        // print!("{:x}", byte);
    // }
}
