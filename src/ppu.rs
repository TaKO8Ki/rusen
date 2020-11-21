use crate::nes::Nes;
use ggez::{graphics, Context};

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

impl Nes {
    pub fn output_block(&mut self, x: u16, y: u16, ctx: &mut Context) {
        let sprite_num = self.ppu.ram[0x2000 + x as usize + y as usize * 0x20];

        let attr = self.ppu.ram[0x23c0 + (x / 4) as usize + (y / 4) as usize * 0x08];
        let pallete: u8;
        if (x % 4 < 2) && (y % 4 < 2) {
            pallete = attr & 0x03
        } else if (x % 4 > 2) && (y % 4 < 2) {
            pallete = (attr & 0x0c) >> 2
        } else if (x % 4 < 2) && (y % 4 > 2) {
            pallete = (attr & 0x30) >> 4
        } else {
            pallete = (attr & 0xc0) >> 6
        }

        let mut sprite_bytes = [0; 16];
        for i in 0..16 {
            sprite_bytes[i] = self.ppu.ram[sprite_num as usize * 16 + i];
        }

        for y in 0..8 {
            for x in 0..8 {
                let color0 = (sprite_bytes[y] & (0x01 << (7 - x))) >> (7 - x);
                let color1 = ((sprite_bytes[y + 8] & (0x01 << (7 - x))) >> (7 - x)) << 1;

                let p = pallete * 4 + color0 + color1;
                let (r, g, b) = (
                    COLORS[self.ppu.ram[0x3f00 + p as usize] as usize][0],
                    COLORS[self.ppu.ram[0x3f00 + p as usize] as usize][1],
                    COLORS[self.ppu.ram[0x3f00 + p as usize] as usize][2],
                );
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect {
                        x: 1.0,
                        y: 1.0,
                        w: 1.0,
                        h: 1.0,
                    },
                    [r as f32, g as f32, b as f32, 0.0].into(),
                )
                .unwrap();
                graphics::draw(
                    ctx,
                    &rectangle,
                    (ggez::mint::Point2 {
                        x: x as f32,
                        y: y as f32,
                    },),
                )
                .unwrap();
                println!("============================================================")
            }
        }
    }

    // fn output_image(bytes [16]byte, pallete byte) (img *image.RGBA) {
    //     img = image.NewRGBA(image.Rect(0, 0, 8, 8))

    //     var x, y uint
    //     for y = 0; y < 8; y++ {
    //         for x = 0; x < 8; x++ {
    //             color0 := (bytes[y] & (0x01 << (7 - x))) >> (7 - x)
    //             color1 := ((bytes[y+8] & (0x01 << (7 - x))) >> (7 - x)) << 1

    //             p := uint(pallete*4) + uint(color0+color1) // パレット番号 + パレット内番号
    //             R, G, B := colors[ppu.RAM[0x3f00+p]][0], colors[ppu.RAM[0x3f00+p]][1], colors[ppu.RAM[0x3f00+p]][2]
    //             img.Set((int)(x), (int)(y), color.RGBA{R, G, B, 0})
    //         }
    //     }
    //     return img
    // }
}
