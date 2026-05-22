use crate::Mmu;
use crate::mmu::memmap::{OBJ_ENABLE_BIT, OBJ_SIZE_BIT, OBP0_ADDR, OBP1_ADDR};
use crate::ppu::tiles::apply_palette_to_pixel;
use crate::ppu::tiles::{TILE_SIDE_LENGTH_PIXELS, get_tile_row};
use crate::ppu::{DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::util::get_bit;
use crate::{Ppu, mmu::memmap::OAM_START};

// OAM scan takes two dots/t-cycles per object, scanning 40 objects in total.

const OBJECT_SIZE_BYTES: u32 = 4;
const SCREEN_BUFFER_X: i32 = 8;
const SCREEN_BUFFER_Y: i32 = 16;

type ObjectDisplay = [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
pub struct OamData {
    pub object_display: ObjectDisplay,
}

impl OamData {
    pub fn new() -> Self {
        OamData {
            object_display: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        }
    }
}

struct ObjectFlags {
    priority: bool,
    xflip: bool,
    yflip: bool,
    palette: bool,
    bank: bool,
}

impl Ppu {
    pub fn tick_oam_scan(&mut self, mmu: &mut Mmu) {
        // An object is scanned only every other PPU tick (40 objects over 80 cycles)
        if !self.mode_dots.is_multiple_of(2) {
            return;
        }

        // The object enable bit lets the game decide whether or not to render an object
        if !self.get_lcdc_flag(OBJ_ENABLE_BIT, mmu) {
            return;
        }

        // An object will only be displayed if a part of it is present at the current scanline
        let object_number: u32 = (self.mode_dots / 2) - 1;
        let object_addr = OAM_START + (object_number * OBJECT_SIZE_BYTES) as u16;
        let (y_position, x_position, mut tile_index, flags) = self.get_oam_bytes(&object_addr, mmu);
        let screen_x = x_position as i32 - SCREEN_BUFFER_X;
        let screen_y = y_position as i32 - SCREEN_BUFFER_Y;
        let tall_object = self.get_lcdc_flag(OBJ_SIZE_BIT, mmu);
        // TODO: It seems that obj size bit is not being enabled early enough.
        // In DMG acid, one row of three 8x16 objects are scanned before the bit is enabled.
        // In particular, during ly == 88, the obj size is not enabled when it should be.
        // I have confirmed in the DMG acid source code that the bit is supposed to be enabled precisely on line 88
        // It seems that the bit is being enabled partyway through line 88
        let object_height = if tall_object { 16 } else { 8 };
        let ly: i32 = self.ly as i32;
        if !((ly >= screen_y) && (ly < screen_y + object_height)) {
            return;
        }

        // Hardware ignores bit 0 of the index for 8x16 objects
        if tall_object {
            tile_index &= 0xFE;
        }

        if object_addr == 0xFE5C {
            // println!("{} : {}", tall_object, ly);
        }

        let mut tile_row_index: i32 = ly - screen_y;

        if flags.yflip {
            tile_row_index = (object_height - 1) - tile_row_index
        }

        // TODO: Add proper logging here
        if tile_row_index < 0 {
            println!("Row index < 0 detected: {}", tile_row_index);
            return;
        }

        // Objects gain rendering priority if they have a lower x-position
        // Or, if they have the same x-position, the first object in memory gets priority
        if self.objects_x.contains(&(x_position as i32)) {
            return;
        }
        self.objects_x[object_number as usize] = x_position as i32;
        // If the program reaches this point, the object will be drawn
        

        let tile_start_addr = self.get_tile_start_addr(tile_index, true, mmu);

        let tile_row_high_byte =
            self.get_tile_row_high_byte(tile_start_addr, tile_row_index as u8, mmu);
        let tile_row_low_byte =
            self.get_tile_row_low_byte(tile_start_addr, tile_row_index as u8, mmu);
        let mut object_row = get_tile_row(tile_row_low_byte, tile_row_high_byte, mmu);

        if flags.xflip {
            object_row.reverse();
        }

        // Palette selection
        let palette: u8 = if flags.palette {
            self.read_byte(OBP1_ADDR, mmu)
        } else {
            self.read_byte(OBP0_ADDR, mmu)
        };

        self.write_row_to_display(&object_row, screen_x, palette);
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
            palette: get_bit(flags_byte, 4),
            bank: get_bit(flags_byte, 3),
        };

        (y_position, x_position, tile_index, flags)
    }

    fn write_row_to_display(
        &mut self,
        row: &[u8; TILE_SIDE_LENGTH_PIXELS],
        mut screen_x: i32,
        palette: u8,
    ) {
        let screen_y = self.ly;
        for pixel in row {
            let pixel_colored = apply_palette_to_pixel(pixel, palette, true);
            if screen_x >= 0 && screen_x < (DISPLAY_WIDTH as i32) {
                self.oam_data.object_display[screen_y as usize][screen_x as usize] = pixel_colored;
            }
            screen_x += 1;
        }
    }
}
