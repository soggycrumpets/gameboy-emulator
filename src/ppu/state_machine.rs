use super::{
    DISPLAY_HEIGHT, DOTS_PER_SCANLINE, HBLANK_MAX_DOTS, OAM_SCAN_DOTS, PIXEL_DRAW_MIN_DOTS, Ppu,
    PpuMode, VBLANK_DOTS, pixel_draw::FetcherState,
};
use crate::mmu::Mmu;
use crate::mmu::memmap::VBLANK_INTERRUPT_BIT;
use crate::ppu::OBJECTS_PER_SCANLINE;

impl Ppu {
    pub fn oam_scan(&mut self, mmu: &mut Mmu) {
        self.tick_oam_scan(mmu);
        // OAMSCAN -> PIXELDRAW
        if OAM_SCAN_DOTS == self.mode_dots {
            self.objects_x = [0; OBJECTS_PER_SCANLINE as usize];
            self.set_mode(PpuMode::PixelDraw, mmu);

            mmu.vram_lock = true;
        }
    }

    pub fn pixel_draw(&mut self, mmu: &mut Mmu) {
        self.tick_fetcher(mmu);
        // PIXELDRAW -> HBLANK
        if PIXEL_DRAW_MIN_DOTS == self.mode_dots {
            self.set_mode(PpuMode::HBlank, mmu);

            // Reset fetcher
            self.fetcher.state = FetcherState::GetTile;
            self.wx_triggered = false;
            self.fetcher.drawing_window = false;

            mmu.vram_lock = false;
            mmu.oam_lock = false;
        }
    }

    pub fn hblank(&mut self, mmu: &mut Mmu) {
        if HBLANK_MAX_DOTS == self.mode_dots {
            // HBLANK -> VBLANK
            if self.ly == DISPLAY_HEIGHT as u8 - 1 {
                self.set_mode(PpuMode::VBlank, mmu);
                mmu.request_interrupt(VBLANK_INTERRUPT_BIT);
            // HBLANK -> OAMSCAN
            } else {
                self.set_mode(PpuMode::OamScan, mmu);
                mmu.oam_lock = true;
            }
            self.inc_ly(mmu);
        }
    }

    pub fn vblank(&mut self, mmu: &mut Mmu) {
        if self.scanline_dots == DOTS_PER_SCANLINE {
            self.inc_ly(mmu);
            // VBLANK -> OAMSCAN
            if VBLANK_DOTS == self.mode_dots {
                self.set_mode(PpuMode::OamScan, mmu);
                mmu.oam_lock = true;
                self.reset_ly(mmu);
                self.frame_complete = true;
            }
        }
    }
}
