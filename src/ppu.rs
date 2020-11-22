use crate::render::GridPosition;
use ggez::graphics::{self, MeshBuilder};

pub const COLORS: [[u8; 3]; 64] = [
    [0x80, 0x80, 0x80],
    [0x00, 0x3D, 0xA6],
    [0x00, 0x12, 0xB0],
    [0x44, 0x00, 0x96],
    [0xA1, 0x00, 0x5E],
    [0xC7, 0x00, 0x28],
    [0xBA, 0x06, 0x00],
    [0x8C, 0x17, 0x00],
    [0x5C, 0x2F, 0x00],
    [0x10, 0x45, 0x00],
    [0x05, 0x4A, 0x00],
    [0x00, 0x47, 0x2E],
    [0x00, 0x41, 0x66],
    [0x00, 0x00, 0x00],
    [0x05, 0x05, 0x05],
    [0x05, 0x05, 0x05],
    [0xC7, 0xC7, 0xC7],
    [0x00, 0x77, 0xFF],
    [0x21, 0x55, 0xFF],
    [0x82, 0x37, 0xFA],
    [0xEB, 0x2F, 0xB5],
    [0xFF, 0x29, 0x50],
    [0xFF, 0x22, 0x00],
    [0xD6, 0x32, 0x00],
    [0xC4, 0x62, 0x00],
    [0x35, 0x80, 0x00],
    [0x05, 0x8F, 0x00],
    [0x00, 0x8A, 0x55],
    [0x00, 0x99, 0xCC],
    [0x21, 0x21, 0x21],
    [0x09, 0x09, 0x09],
    [0x09, 0x09, 0x09],
    [0xFF, 0xFF, 0xFF],
    [0x0F, 0xD7, 0xFF],
    [0x69, 0xA2, 0xFF],
    [0xD4, 0x80, 0xFF],
    [0xFF, 0x45, 0xF3],
    [0xFF, 0x61, 0x8B],
    [0xFF, 0x88, 0x33],
    [0xFF, 0x9C, 0x12],
    [0xFA, 0xBC, 0x20],
    [0x9F, 0xE3, 0x0E],
    [0x2B, 0xF0, 0x35],
    [0x0C, 0xF0, 0xA4],
    [0x05, 0xFB, 0xFF],
    [0x5E, 0x5E, 0x5E],
    [0x0D, 0x0D, 0x0D],
    [0x0D, 0x0D, 0x0D],
    [0xFF, 0xFF, 0xFF],
    [0xA6, 0xFC, 0xFF],
    [0xB3, 0xEC, 0xFF],
    [0xDA, 0xAB, 0xEB],
    [0xFF, 0xA8, 0xF9],
    [0xFF, 0xAB, 0xB3],
    [0xFF, 0xD2, 0xB0],
    [0xFF, 0xEF, 0xA6],
    [0xFF, 0xF7, 0x9C],
    [0xD7, 0xE8, 0x95],
    [0xA6, 0xED, 0xAF],
    [0xA2, 0xF2, 0xDA],
    [0x99, 0xFF, 0xFC],
    [0xDD, 0xDD, 0xDD],
    [0x11, 0x11, 0x11],
    [0x11, 0x11, 0x11],
];

pub struct Ppu {
    pub ram: [u8; 0x4000],
    pub s_ram: [u8; 0x100],
    pub mirror: bool,
    pub ptr: u16,
    pub ppudata_buf: u8,
    pub scroll: [u8; 2],
    pub scroll_flag: bool,
    pub raster: u16,
    pub cycle: usize,
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
            cycle: 0,
        }
    }
}

impl Ppu {
    pub fn step(&mut self, x: u16, y: u16, b_x: u16, b_y: u16, mesh: &mut MeshBuilder) {
        let sprite_num = self.ram[0x2000 + b_x as usize + b_y as usize * 0x20];
        let attr = self.ram[0x23c0 + b_x as usize / 4 + b_y as usize / 4 * 0x08];
        let pallete: u8;
        if (b_x % 4 < 2) && (b_y % 4 < 2) {
            pallete = attr & 0x03
        } else if (b_x % 4 > 2) && (b_y % 4 < 2) {
            pallete = (attr & 0x0c) >> 2
        } else if (b_x % 4 < 2) && (b_y % 4 > 2) {
            pallete = (attr & 0x30) >> 4
        } else {
            pallete = (attr & 0xc0) >> 6
        }

        let mut sprite_bytes = [0; 16];
        for i in 0..16 {
            sprite_bytes[i] = self.ram[sprite_num as usize * 16 + i];
        }
        let color0 = (sprite_bytes[y as usize] & (0x01 << (7 - x))) >> (7 - x);
        let color1 = ((sprite_bytes[y as usize + 8] & (0x01 << (7 - x))) >> (7 - x)) << 1;
        let p = pallete * 4 + color0 + color1;
        let (r, g, b) = (
            COLORS[self.ram[0x3f00 + p as usize] as usize][0],
            COLORS[self.ram[0x3f00 + p as usize] as usize][1],
            COLORS[self.ram[0x3f00 + p as usize] as usize][2],
        );
        mesh.rectangle(
            graphics::DrawMode::fill(),
            GridPosition {
                x,
                y: y as u16,
                b_x,
                b_y,
            }
            .into(),
            graphics::Color::from_rgb(r, g, b),
        );
    }
}
