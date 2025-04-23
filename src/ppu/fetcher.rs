use super::*;

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
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            state: FetcherState::GetTile,
            tile_x: 0,
            tile_y: 0,
        }
    }
}

impl Ppu {
    fn tick_fetcher(&mut self) {
        match self.fetcher_state {
            FetcherState::GetTile => self.fetcher_get_tile(),
            FetcherState::GetTileDataHigh => self.fetcher_get_tile_data_high(),
            FetcherState::GetTileDataLow => self.fetcher_get_tile_data_low(),
            FetcherState::Sleep => self.fetcher_sleep(),
            FetcherState::Push => self.fetcher_push(),
        }
    }

    fn fetcher_get_tile(&mut self) {
        let bg_tile_map = self.get_lcdc_flag(BG_TILE_MAP_BIT);
        let window_tile_map = self.get_lcdc_flag(WINDOW_TILE_MAP_BIT);

        self.update_wx();
        let drawing_window = self.wx_triggered && self.wy_triggered;

        let tilemap_addr: u16 =
            if (bg_tile_map && drawing_window) || (window_tile_map && !drawing_window) {
                0x9C00
            } else {
                0x9800
            };

        let (x, y) = if window_tile_map {
            (self.lx, self.ly)
        } else {
            self.fetcher.tile_y = self.ly;
            let scx = self.read_byte(SCX_ADDR);
            let scy = self.read_byte(SCY_ADDR);
            ((self.lx + (scx / 8)) & 0x1F, (self.ly + scy) & 255)
        };
    }
    fn fetcher_get_tile_data_high(&self) {}

    fn fetcher_get_tile_data_low(&self) {}

    fn fetcher_sleep(&self) {}

    fn fetcher_push(&self) {}

    fn update_wx(&mut self) {
        let wx = self.read_byte(WX_ADDR);
        self.wx_triggered = (self.lx + 7);
    }
}
