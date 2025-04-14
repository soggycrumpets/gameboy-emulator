pub struct LcdControl {
    lcd_and_ppu_enabled: bool,
    window_tile_map: bool,
    window_enable: bool,
    bg_and_window_tiles: bool,
    bg_tile_map: bool,
    obj_size: bool,
    obj_enable: bool,
    bg_and_window_enable: bool,
}

pub struct LcdStatus {
    lyc_int_select: bool, // Bit 6
    mode_2_int_select: bool, // Bit 5
    mode_1_int_select: bool, // Bit 4
    mode_0_int_select: bool, // Bit 3
    lyc_equals_ly: bool, // Bit 2
    ppu_mode: [bool; 2], // Bit 0, 1
}

// Window Position Y
pub struct Wy {

}

impl LcdControl {
    pub fn new() -> Self {
        LcdControl {
            lcd_and_ppu_enabled: false,
            window_tile_map: false,
            window_enable: false,
            bg_and_window_tiles: false,
            bg_tile_map: false,
            obj_size: false,
            obj_enable: false,
            bg_and_window_enable: false,
        }
    }
}

impl LcdStatus {
    pub fn new() -> Self {
        LcdStatus {
            lyc_int_select: false,
            mode_2_int_select: false,
            mode_1_int_select: false,
            mode_0_int_select: false,
            lyc_equals_ly: false,
            ppu_mode: [false; 2],
        }
    }
}
