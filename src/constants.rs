pub const PROGRAM_START_ADDR: u16 = 0x0100;
pub const TOP_OF_STACK_ADDRESS: u16 = 0xFFFE;

pub const ROM_FOLDER_PATH: &str = "./roms/";

// These contain the minimum number of tick cycles that each instruction can take to perform
pub const UNPREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../data/unprefixed_instruction_t_cycle_table.dat");

pub const PREFIXED_INSTRUCTION_T_CYCLE_TABLE: &[u8; 256] =
    include_bytes!("../data/prefixed_instruction_t_cycle_table.dat");
