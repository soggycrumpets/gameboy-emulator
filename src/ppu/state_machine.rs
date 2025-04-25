use crate::mmu::memmap::VBLANK_INTERRUPT_BIT;

use super::{
    HBLANK_MAX_T_CYCLES, OAM_SCAN_T_CYCLES, PIXEL_DRAW_MIN_T_CYCLES, Ppu, PpuMode,
    T_CYCLES_PER_FRAME, VBLANK_T_CYCLES,
};

impl Ppu {
    pub fn oam_scan(&mut self) {
        // OAMSCAN -> PIXELDRAW
        if self.scanline_t_cycle_count == OAM_SCAN_T_CYCLES {
            self.lx = 0;

            self.set_mode(PpuMode::PixelDraw);
            self.mmu.borrow_mut().vram_lock = true;
        }
    }

    pub fn pixel_draw(&mut self) {
        // PIXELDRAW -> HBLANK
        self.tick_fetcher();
        if self.scanline_t_cycle_count == OAM_SCAN_T_CYCLES + PIXEL_DRAW_MIN_T_CYCLES {
            self.set_mode(PpuMode::HBlank);
            self.lx = 0;
            self.fetcher.dots = 0;
            self.mmu.borrow_mut().vram_lock = false;
            self.mmu.borrow_mut().oam_lock = false;
            self.wx_triggered = false;
        }
    }

    pub fn hblank(&mut self) {
        // HBLANK -> VBLANK
        if self.frame_t_cycle_count == T_CYCLES_PER_FRAME - VBLANK_T_CYCLES {
            self.set_mode(PpuMode::VBlank);
            self.inc_ly();
            self.mmu
                .borrow_mut()
                .request_interrupt(VBLANK_INTERRUPT_BIT);
            self.update_wy();
        // HBLANK -> OAMSCAN
        } else if self.scanline_t_cycle_count
            == OAM_SCAN_T_CYCLES + PIXEL_DRAW_MIN_T_CYCLES + HBLANK_MAX_T_CYCLES
        {
            self.set_mode(PpuMode::OamScan);
            self.inc_ly();
            self.mmu.borrow_mut().oam_lock = true;
            self.update_wy();
        }
    }

    pub fn vblank(&mut self) {
        if self.frame_t_cycle_count == T_CYCLES_PER_FRAME {
            // println!("VBLANK -> OAM");
            self.set_mode(PpuMode::OamScan);
            self.mmu.borrow_mut().oam_lock = true;
            self.reset_ly();
            self.wy_counter = 0;
            self.wy_triggered = false;
            self.update_wy();
        }
    }
}
