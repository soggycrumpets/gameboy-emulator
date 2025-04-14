enum PpuMode {
    HBlank,
    VBlank,
    OamScan,
    HDraw,
}

pub struct Ppu {
    mode: PpuMode,
}