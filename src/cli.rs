use crate::do_cpu_test;
use std::num::ParseIntError;

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

pub enum Command {
    Test(u32),
    Run(String),
}
pub fn check_cli_inputs() -> bool {
    if let Some(input) = parse_cli_inputs() {
        match input {
            Command::Test(test_number) => {
                let test_path = match test_number {
                    1 => TEST_CPU_1_PATH,
                    2 => TEST_CPU_2_PATH,
                    3 => TEST_CPU_3_PATH,
                    4 => TEST_CPU_4_PATH,
                    5 => TEST_CPU_5_PATH,
                    6 => TEST_CPU_6_PATH,
                    7 => TEST_CPU_7_PATH,
                    8 => TEST_CPU_8_PATH,
                    9 => TEST_CPU_9_PATH,
                    10 => TEST_CPU_10_PATH,
                    11 => TEST_CPU_11_PATH,
                    _ => panic!("Ivalid test: {}", test_number),
                };
                do_cpu_test(test_path);
            }
            Command::Run(rom) => unimplemented!("Run Command"),
        }
        return true;
    }
    false
}

pub fn parse_cli_inputs() -> Option<Command> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return None;
    }

    let arg = &args[1].to_lowercase();

    match arg.as_str() {
        "test" => check_test_number_arg(&args).map(Command::Test),
        // "rom" => return get_rom(arg),
        _ => None,
    }
}

fn check_test_number_arg(args: &[String]) -> Option<u32> {
    if args.len() < 3 {
        return None;
    }

    let arg = &args[2];

    arg.parse::<u32>().ok()
}

fn check_rom_name_art() {
    
}