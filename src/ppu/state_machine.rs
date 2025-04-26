use crate::mmu::memmap::VBLANK_INTERRUPT_BIT;

use super::{
    HBLANK_MAX_DOTS, OAM_SCAN_DOTS, PIXEL_DRAW_MIN_DOTS, Ppu, PpuMode,
    FRAME_DOTS, VBLANK_DOTS,
};

impl Ppu {
    pub fn oam_scan(&mut self) {
        // OAMSCAN -> PIXELDRAW
        if self.mode_dots == OAM_SCAN_DOTS {
            self.set_mode(PpuMode::PixelDraw);
            self.lx = 0;

            self.mmu.borrow_mut().vram_lock = true;
        }
    }

    pub fn pixel_draw(&mut self) {
        // PIXELDRAW -> HBLANK
        self.tick_fetcher();
        if self.mode_dots == PIXEL_DRAW_MIN_DOTS {
            self.set_mode(PpuMode::HBlank);
            self.mmu.borrow_mut().vram_lock = false;
            self.mmu.borrow_mut().oam_lock = false;
            self.wx_triggered = false;
            self.fetcher.drawing_window = false;
        }
    }

    pub fn hblank(&mut self) {
        // HBLANK -> VBLANK
        if self.frame_dots == FRAME_DOTS - VBLANK_DOTS {
            self.set_mode(PpuMode::VBlank);
            self.mmu
                .borrow_mut()
                .request_interrupt(VBLANK_INTERRUPT_BIT);
        // HBLANK -> OAMSCAN
        } else if self.mode_dots == HBLANK_MAX_DOTS
        {
            self.set_mode(PpuMode::OamScan);
            self.mmu.borrow_mut().oam_lock = true;
        }
    }

    pub fn vblank(&mut self) {
        if self.mode_dots == VBLANK_DOTS {
            // println!("VBLANK -> OAM");
            self.set_mode(PpuMode::OamScan);
            self.mmu.borrow_mut().oam_lock = true;
        }
    }
}
