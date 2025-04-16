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

const TEST_INSTR_TIMING_PATH: &str = "./test-roms/instr_timing.gb";
const TEST_MEM_TIMING_PATH: &str = "./test-roms/mem_timing.gb";

const DMG_ACID_PATH: &str = "./test-roms/dmg-acid2.gb";

const TETRIS_ROM_PATH: &str = "./roms/tetris.gb";
const DEFAULT_ROM_PATH: &str = TETRIS_ROM_PATH;

pub enum Command {
    // Test(String),
    Rom(String),
    Debug(String),
}

pub fn parse_cli_inputs() -> Command {
    let mut args: Vec<String> = std::env::args().collect();
    args.reverse(); // This way, the args can be popped from the back in order
    args.pop(); // Discard the first CLI arg (it's just the path to the executable)

    let arg = args.pop();
    if arg.is_none() {
        return Command::Rom(DEFAULT_ROM_PATH.to_string());
    }

    match arg.unwrap().as_str() {
        "debug" => Command::Debug(parse_rom_arg(args)),
        "rom" => Command::Rom(parse_rom_arg(args)),
        _ => Command::Rom(parse_rom_arg(args)),
    }
}

fn parse_rom_arg(mut args: Vec<String>) -> String {
    let arg = args.pop();
    if arg.is_none() {
        return map_rom_name_to_path("");
    }

    map_rom_name_to_path(&arg.unwrap())
}

fn map_rom_name_to_path(name: &str) -> String {
    match name {
        "tetris" => TETRIS_ROM_PATH,
        "cpu1" => TEST_CPU_1_PATH,
        "cpu2" => TEST_CPU_2_PATH,
        "cpu3" => TEST_CPU_3_PATH,
        "cpu4" => TEST_CPU_4_PATH,
        "cpu5" => TEST_CPU_5_PATH,
        "cpu6" => TEST_CPU_6_PATH,
        "cpu7" => TEST_CPU_7_PATH,
        "cpu8" => TEST_CPU_8_PATH,
        "cpu9" => TEST_CPU_9_PATH,
        "cpu10" => TEST_CPU_10_PATH,
        "cpu11" => TEST_CPU_11_PATH,
        "acid" => DMG_ACID_PATH,
        "instrtiming" => TEST_INSTR_TIMING_PATH,
        "memtiming" => TEST_MEM_TIMING_PATH,
        _ => {
            name
        }
    }
    .to_string()
}
