use super::{
    tiles::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS, get_tile_row},
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

pub struct Fetcher {
    state: FetcherState,
    tile_x: u8,
    tile_y: u8,
    y: u8,

    pub drawing_window: bool,

    tile_addr: u16,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            state: FetcherState::GetTile,
            tile_x: 0,
            tile_y: 0,
            y: 0,

            drawing_window: false,

            tile_addr: 0x0000,
            tile_data_low: 0,
            tile_data_high: 0,
        }
    }
}

impl Ppu {
    pub fn tick_fetcher(&mut self) {
        if self.mode_dots % 2 != 0 {
            return;
        }

        if self.mode_dots > 160 {
            return;
        }

        match self.fetcher.state {
            FetcherState::GetTile => {
                self.fetcher_get_tile();
                self.fetcher.state = FetcherState::GetTileDataLow;
            }
            FetcherState::GetTileDataLow => {
                self.fetcher_get_tile_data(false);
                self.fetcher.state = FetcherState::GetTileDataHigh;
            }
            FetcherState::GetTileDataHigh => {
                self.fetcher_get_tile_data(true);
                self.fetcher.state = FetcherState::Push;
            }
            FetcherState::Sleep => self.fetcher_sleep(),
            FetcherState::Push => {
                self.fetcher_push();
                self.fetcher.state = FetcherState::GetTile;
                self.lx += 8;
            }
        }
    }

    fn fetcher_get_tile(&mut self) {
        let bg_tile_map = self.get_lcdc_flag(BG_TILE_MAP_BIT);
        let window_tile_map = self.get_lcdc_flag(WINDOW_TILE_MAP_BIT);

        self.set_lcdc_flag(WINDOW_ENABLE_BIT, true);

        self.update_wx();
        self.fetcher.drawing_window = self.wx_triggered && self.wy_triggered;

        let tilemap_base_addr = if !self.fetcher.drawing_window && bg_tile_map
        || self.fetcher.drawing_window && window_tile_map {
            TILEMAP_2_ADDR
        } else {
            TILEMAP_1_ADDR
        };

        (self.fetcher.tile_x, self.fetcher.y) = if self.fetcher.drawing_window {
            let wx = self.read_byte(WX_ADDR).wrapping_sub(7);
            let wy = self.wy_counter;
            (self.lx.wrapping_sub(wx) / 8, wy)
        } else {
            let scx = self.read_byte(SCX_ADDR);
            let scy = self.read_byte(SCY_ADDR);
            (
                (self.lx / 8 + (scx / 8)) & 0x1F,
                (self.ly as u16 + scy as u16) as u8,
            )
        };

        self.fetcher.tile_y = self.fetcher.y / 8;

        let tilemap_addr = tilemap_base_addr
            + (self.fetcher.tile_y as u16 * TILEMAP_WIDTH)
            + (self.fetcher.tile_x) as u16;

        let tile_index = self.read_byte(tilemap_addr);

        self.fetcher.tile_addr = self.get_tile_start_addr(tile_index);
    }

    fn fetcher_get_tile_data(&mut self, high: bool) {
        let tile_start_addr = self.fetcher.tile_addr;
        let row_index = self.fetcher.y % TILE_HEIGHT_IN_PIXELS as u8;

        if high {
            self.fetcher.tile_data_high =
                self.read_byte(tile_start_addr + (row_index as u16 * 2) + 1);
        } else {
            self.fetcher.tile_data_low = self.read_byte(tile_start_addr + (row_index as u16 * 2));
        }
    }

    fn fetcher_sleep(&self) {}

    fn fetcher_push(&mut self) {
        let tile_row = get_tile_row(self.fetcher.tile_data_low, self.fetcher.tile_data_high);
        let row = self.ly as usize;
        let col = self.lx as usize;
        for (i, pixel) in tile_row.iter().enumerate() {
            self.display[row][col + i] = *pixel;
        }
    }

    fn update_wx(&mut self) {
        let wx = self.read_byte(WX_ADDR);
        if (self.lx) == wx.wrapping_sub(7) {
            self.wx_triggered = true;
        }
    }
}
