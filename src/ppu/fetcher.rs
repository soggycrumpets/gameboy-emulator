use super::{
    tiles::{TILE_HEIGHT_IN_PIXELS, TILE_WIDTH_IN_PIXELS, get_tile_row},
    *,
};

const TILEMAP_1_ADDR: u16 = 0x9800;
const TILEMAP_2_ADDR: u16 = 0x9C00;
const TILEMAP_WIDTH: u8 = 32;

#[derive(Debug)]
pub enum FetcherState {
    GetTile,
    GetTileDataHigh,
    GetTileDataLow,
    Sleep,
    Push,
}

pub struct Fetcher {
    pub dots: u32,

    state: FetcherState,
    tile_x: u8,
    tile_y: u8,

    tile_row: u8,

    tile_addr: u16,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher {
            dots: 0,

            state: FetcherState::GetTile,
            tile_x: 0,
            tile_y: 0,

            tile_row: 0,

            tile_addr: 0x0000,
            tile_data_low: 0,
            tile_data_high: 0,
        }
    }
}

impl Ppu {
    pub fn tick_fetcher(&mut self) {
        self.fetcher.dots += 1;
        if self.fetcher.dots % 2 != 0 {
            return;
        }

        if self.fetcher.dots >= 160 {
            return;
        }

        match self.fetcher.state {
            FetcherState::GetTile => {
                self.fetcher_get_tile();
                self.fetcher.state = FetcherState::GetTileDataLow;
                self.lx += 8;
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
            }
        }
    }

    fn fetcher_get_tile(&mut self) {
        let bg_tile_map = self.get_lcdc_flag(BG_TILE_MAP_BIT);
        let window_tile_map = self.get_lcdc_flag(WINDOW_TILE_MAP_BIT);

        self.update_wx();
        let drawing_window = self.wx_triggered && self.wy_triggered;

        let tilemap_base_addr =
            if (bg_tile_map && drawing_window) || (window_tile_map && !drawing_window) {
                TILEMAP_2_ADDR
            } else {
                TILEMAP_1_ADDR
            };

        let (tile_x, tile_y) = if window_tile_map {
            (self.lx, self.ly)
        } else {
            let scx = self.read_byte(SCX_ADDR);
            let scy = self.read_byte(SCY_ADDR);
            ((self.lx + (scx / 8)) & 0x1F, (self.ly + scy) & 0xFF)
        };

        let tilemap_addr =
            tilemap_base_addr + (tile_y as u16 * TILEMAP_WIDTH as u16) + tile_x as u16;
        let tile_index = self.read_byte(tilemap_addr);

        println!("{:02x}", tile_index);

        self.fetcher.tile_addr = self.get_tile_start_addr(tile_index);
    }

    fn fetcher_get_tile_data(&mut self, high: bool) {
        let tile_start_addr = self.fetcher.tile_addr;
        let row = self.ly;

        if high {
            self.fetcher.tile_data_high = self.read_byte(tile_start_addr + (row as u16 * 2) + 1);
            print!("{:04x} : ", tile_start_addr + (row as u16 * 2) + 1);
            println!("{:02x}", self.fetcher.tile_data_high);
        } else {
            self.fetcher.tile_data_low = self.read_byte(tile_start_addr + (row as u16 * 2));
            print!("{:04x} : ", tile_start_addr + (row as u16 * 2));
            println!("{:02x}", self.fetcher.tile_data_low);
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
        self.wx_triggered = (self.lx + 7) == wx;
    }
}
