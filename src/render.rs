use crate::nes::Nes;
use crate::ppu::COLORS;
use ggez::graphics::{self, MeshBuilder};
use ggez::{event, Context, GameResult};

const WIDTH: u16 = 256;
const HEIGHT: u16 = 240;

struct GridPosition {
    x: u16,
    y: u16,
    p_x: u16,
    p_y: u16,
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect {
            x: (pos.p_x * 8 + pos.x) as f32,
            y: (HEIGHT as f64 - 20.0 - pos.p_y as f64 * 8.0 + pos.y as f64) as f32,
            w: 1.0,
            h: 1.0,
        }
    }
}

impl event::EventHandler for Nes {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.set_v_blank();
        let mut mesh = MeshBuilder::new();
        for p_y in 0..HEIGHT / 8 {
            for p_x in 0..WIDTH / 8 {
                let sprite_num = self.ppu.ram[0x2000 + p_x as usize + p_y as usize * 0x20];

                let attr = self.ppu.ram[0x23c0 + p_x as usize / 4 + p_y as usize / 4 * 0x08];
                let pallete: u8;
                if (p_x % 4 < 2) && (p_y % 4 < 2) {
                    pallete = attr & 0x03
                } else if (p_x % 4 > 2) && (p_y % 4 < 2) {
                    pallete = (attr & 0x0c) >> 2
                } else if (p_x % 4 < 2) && (p_y % 4 > 2) {
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
                        mesh.rectangle(
                            graphics::DrawMode::fill(),
                            GridPosition {
                                x,
                                y: y as u16,
                                p_x,
                                p_y,
                            }
                            .into(),
                            graphics::Color::from_rgb(r, g, b),
                        );
                    }
                }
            }
        }
        let mesh = mesh.build(ctx)?;
        graphics::draw(ctx, &mesh, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        self.clear_v_blank();
        graphics::present(ctx)?;
        Ok(())
    }
}

impl Nes {
    pub fn run(&mut self) -> GameResult {
        let (ctx, events_loop) = &mut ggez::ContextBuilder::new("nes emulator", "TaKO8Ki")
            .window_setup(ggez::conf::WindowSetup::default().title("hello world"))
            .window_mode(
                ggez::conf::WindowMode::default()
                    .dimensions(WIDTH as f32, HEIGHT as f32)
                    .resizable(true),
            )
            .build()?;
        event::run(ctx, events_loop, self)
    }
}
