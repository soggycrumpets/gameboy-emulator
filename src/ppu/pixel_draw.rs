use super::{
    tiles::{TILE_SIDE_LENGTH_PIXELS, get_tile_row},
    *,
};

const TILEMAP_1_ADDR: u16 = 0x9800;
const TILEMAP_2_ADDR: u16 = 0x9C00;
const TILEMAP_WIDTH: u16 = 32;

#[derive(Debug)]
pub enum FetcherState {
    GetTile,
    GetTileDataHigh,
    GetTileDataLow,
    Sleep,
    Push,
}

/// The fetcher is a state machine within the PPU state machine.
/// It retrieves pixel data from memory to be drawn to the screen, one tile row (8 pixels) at a time.
/// It operates over the duration of the pixel-draw state, retrieving one scanline worth of pixels.
#[derive(Debug)]
pub struct Fetcher {
    pub state: FetcherState,

    x: u8,
    y: u8,
    tile_x: u8,
    tile_y: u8,

    pub drawing_window: bool,

    tile_addr: u16,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            state: FetcherState::GetTile,

            x: 0,
            y: 0,
            tile_x: 0,
            tile_y: 0,

            drawing_window: false,

            tile_addr: 0x0000,
            tile_data_low: 0,
            tile_data_high: 0,
        }
    }
}

impl Ppu {
    /// This implementation of the fetcher, at least for now, is greatly simplified.
    /// Each state is completing within a constant time of 2 dots/t-cycles.
    /// The fetcher normally works with a pixel FIFO (first-in, first-out) to render
    /// to the screen, but this implementation does not use the FIFO.
    /// The way it works now: 8 t-cycles to draw 8 pixels, 160 t-cycles to draw a scanline.
    /// The fetcher progresses one state every other t-cycle.
    pub fn tick_fetcher(&mut self, mmu: &mut Mmu) {
        if !self.mode_dots.is_multiple_of(2) {
            return;
        }

        if self.mode_dots > DISPLAY_WIDTH as u32 {
            return;
        }

        match self.fetcher.state {
            FetcherState::GetTile => {
                self.fetcher_get_tile(mmu);
                self.fetcher.state = FetcherState::GetTileDataLow;
            }
            FetcherState::GetTileDataLow => {
                self.fetcher_get_tile_data(false, mmu);
                self.fetcher.state = FetcherState::GetTileDataHigh;
            }
            FetcherState::GetTileDataHigh => {
                self.fetcher_get_tile_data(true, mmu);
                self.fetcher.state = FetcherState::Push;
            }
            FetcherState::Sleep => self.fetcher_sleep(),
            FetcherState::Push => {
                self.fetcher_push(mmu);
                self.fetcher.state = FetcherState::GetTile;
                self.lx += TILE_SIDE_LENGTH_PIXELS as u8;
            }
        }
    }

    fn fetcher_get_tile(&mut self, mmu: &mut Mmu) {
        let bg_tile_map = self.get_lcdc_flag(BG_TILE_MAP_BIT, mmu);
        let window_tile_map = self.get_lcdc_flag(WINDOW_TILE_MAP_BIT, mmu);

        let window_enable = self.get_lcdc_flag(WINDOW_ENABLE_BIT, mmu);

        self.update_wx(mmu);
        self.fetcher.drawing_window = self.wx_triggered && self.wy_triggered && window_enable;
        if self.fetcher.drawing_window {
            self.window_drawn_this_scanline = true;
        }

        // The two tilemap addresses can be accessed both in background mode and in window mode.
        // Window and background mode each have a bit that determines which tilemap they will use.
        let tilemap_base_addr = if !self.fetcher.drawing_window && bg_tile_map
            || self.fetcher.drawing_window && window_tile_map
        {
            TILEMAP_2_ADDR
        } else {
            TILEMAP_1_ADDR
        };

        // WX is subtracted from LX because LX = WX (accounting for the offset of 7) should grab
        // the leftmost window tile from memory. LX = WX + 1 should grab the next, etc.
        // WY works in the same way, as the WY counter is externally keeping track of the window
        // tile's current y-position in memory. wy_counter = 0 will render the topmost window tile,
        // wy_counter = 1 will render the next, etc.
        (self.fetcher.x, self.fetcher.y) = if self.fetcher.drawing_window {
            let wx = self.read_byte(WX_ADDR, mmu).wrapping_sub(7);
            let wy = self.wy_counter;
            (self.lx.wrapping_sub(wx), wy)
        } else {
            let scx = self.read_byte(SCX_ADDR, mmu);
            let scy = self.read_byte(SCY_ADDR, mmu);
            (self.lx.wrapping_add(scx), self.ly.wrapping_add(scy))
        };

        self.fetcher.tile_x = self.fetcher.x / 8;
        self.fetcher.tile_y = self.fetcher.y / 8;

        let tilemap_addr = tilemap_base_addr
            + (self.fetcher.tile_y as u16 * TILEMAP_WIDTH)
            + (self.fetcher.tile_x) as u16;

        let tile_index = self.read_byte(tilemap_addr, mmu);

        self.fetcher.tile_addr = self.get_tile_start_addr(tile_index, false, mmu);
    }

    fn fetcher_get_tile_data(&mut self, high: bool, mmu: &mut Mmu) {
        let tile_start_addr = self.fetcher.tile_addr;
        let row_index = self.fetcher.y % TILE_SIDE_LENGTH_PIXELS as u8;

        if high {
            self.fetcher.tile_data_high =
                self.get_tile_row_high_byte(tile_start_addr, row_index, mmu);
        } else {
            self.fetcher.tile_data_low =
                self.get_tile_row_low_byte(tile_start_addr, row_index, mmu);
        }
    }

    fn fetcher_sleep(&self) {}

    fn fetcher_push(&mut self, mmu: &mut Mmu) {
        let mut tile_row = get_tile_row(self.fetcher.tile_data_low, self.fetcher.tile_data_high, mmu);
        let row = self.ly as usize;
        let col = self.lx as usize;

        // If bg and window isn't enabled, the pixels are replaced with all 0
        let bg_and_window_enable = self.get_lcdc_flag(BG_AND_WINDOW_ENABLE_BIT, mmu);
        tile_row = if bg_and_window_enable {
            tile_row
        } else {
            [0; 8]
        };

        for (i, pixel) in tile_row.iter().enumerate() {
            self.display[row][col + i] = *pixel;
        }
    }

    /// WX = 7 starts rendering the window at the left of the screen, so WX = 0 is one tile
    /// offscreen to the left. LX = 0, on the other hand, starts at the left of the screen as you
    /// would expect. This means that any time you compare the two, you need to either add 7 to LX
    /// or subtract 7 from WX to ensure that they are both measured from the same point.
    fn update_wx(&mut self, mmu: &mut Mmu) {
        let wx = self.read_byte(WX_ADDR, mmu);
        if (self.lx) == wx.wrapping_sub(7) {
            self.wx_triggered = true;
        }
    }
}
