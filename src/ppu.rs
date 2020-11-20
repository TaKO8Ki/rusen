use crate::nes::Nes;

pub struct Ppu {
    pub ram: [u8; 0x4000],
    pub s_ram: [u8; 0x100],
    pub mirror: bool,
    pub ptr: u16,
    pub ppudata_buf: u8,
    pub scroll: [u8; 2],
    pub scroll_flag: bool,
    pub raster: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Ppu {
            ram: [0; 0x4000],
            s_ram: [0; 0x100],
            mirror: false,
            ptr: 1,
            ppudata_buf: 1,
            scroll: [0; 2],
            scroll_flag: false,
            raster: 3,
        }
    }
}

impl Nes {}
