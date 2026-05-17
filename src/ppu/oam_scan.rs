use crate::Mmu;
use crate::mmu::memmap::OBJ_ENABLE_BIT;
use crate::ppu::tiles::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS, get_tile_row};
use crate::ppu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::util::get_bit;
use crate::{Ppu, mmu::memmap::OAM_START};

// OAM scan takes two dots/t-cycles per object, scanning 40 objects in total.

// TODO:
// - Implement 8x16 object mode
// - Implement priority
// - Fix nose in DMG acid

const OBJECT_SIZE_BYTES: u32 = 4;
const SCREEN_BUFFER_X: u32 = 8;
const SCREEN_BUFFER_Y: u32 = 16;

type ObjectDisplay = [[Option<u8>; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
pub struct OamData {
    pub object_display: ObjectDisplay,
}

impl OamData {
    pub fn new() -> Self {
        OamData {
            object_display: [[None; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }
}

struct ObjectFlags {
    priority: bool,
    xflip: bool,
    yflip: bool,
}

impl Ppu {
    pub fn tick_oam_scan(&mut self, mmu: &mut Mmu) {
        if !self.mode_dots.is_multiple_of(2) {
            return;
        }

        let object_number = (self.mode_dots / 2) - 1;
        let object_addr = OAM_START + (object_number * OBJECT_SIZE_BYTES) as u16;
        let object_enable = self.get_lcdc_flag(OBJ_ENABLE_BIT, mmu);
        let (y_position, x_position, tile_index, flags) = self.get_oam_bytes(&object_addr, mmu);

        let screen_x = x_position as i32 - SCREEN_BUFFER_X as i32;
        let screen_y = y_position as i32 - SCREEN_BUFFER_Y as i32;

        let tile_start_addr = self.get_tile_start_addr(tile_index, true, mmu);

        let object_height = 8;
        let ly: i32 = self.ly as i32;

        // Display the object only if it is on the current scanline
        if !((ly >= screen_y) && (ly < screen_y + object_height)) {
            return;
        }
        // The object enable bit lets the program decide whether or not to render an object
        if !object_enable {
            return;
        }

        let mut tile_row_index: i32 = ly - screen_y;

        // TODO: Add proper logging here
        if tile_row_index < 0 {
            println!("Row index < 0 detected: {}", tile_row_index);
            return;
        }

        if flags.yflip {
            tile_row_index = (TILE_HEIGHT_IN_PIXELS as i32 - 1) - tile_row_index;
        }

        let tile_row_high_byte =
            self.get_tile_row_high_byte(tile_start_addr, tile_row_index as u8, mmu);
        let tile_row_low_byte =
            self.get_tile_row_low_byte(tile_start_addr, tile_row_index as u8, mmu);
        let mut object_row = get_tile_row(tile_row_low_byte, tile_row_high_byte);

        if flags.xflip {
            object_row.reverse();
        }

        self.write_row_to_display(&object_row, screen_x);

        // println!(
        //     "{}: {:0x}-{:0x} | x: {}, y: {}, idx: {}, tile addr: 0x{:0x} priority: {}, xflip: {}, yflip: {}",
        //     object_number,
        //     object_addr,
        //     object_addr + OBJECT_SIZE_BYTES as u16 - 1,
        //     x_position,
        //     y_position,
        //     tile_index,
        //     tile_start_addr,
        //     flags.priority,
        //     flags.xflip,
        //     flags.yflip,
        // );
    }

    fn get_oam_bytes(&mut self, addr: &u16, mmu: &mut Mmu) -> (u8, u8, u8, ObjectFlags) {
        let y_position = self.read_byte(*addr, mmu);
        let x_position = self.read_byte(*addr + 1, mmu);
        let tile_index = self.read_byte(*addr + 2, mmu);
        let flags_byte = self.read_byte(*addr + 3, mmu);
        let flags = ObjectFlags {
            priority: get_bit(flags_byte, 7),
            yflip: get_bit(flags_byte, 6),
            xflip: get_bit(flags_byte, 5),
        };

        (y_position, x_position, tile_index, flags)
    }

    fn write_row_to_display(&mut self, row: &[u8; TILE_WIDTH_IN_PIXELS], mut screen_x: i32) {
        let screen_y = self.ly;
        for pixel in row {

            if screen_x >= 0 && screen_x < (DISPLAY_WIDTH as i32) {
                self.oam_data.object_display[screen_y as usize][screen_x as usize] = Some(*pixel);
            }
            screen_x += 1;
        }
    }
}
