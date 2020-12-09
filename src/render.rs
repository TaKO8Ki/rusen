use crate::nes::Nes;
use ggez::graphics::{self, MeshBuilder};
use ggez::{event, Context, GameResult};

const WIDTH: u16 = 256;
const HEIGHT: u16 = 240;

pub struct GridPosition {
    pub x: u16,
    pub y: u16,
    pub b_x: u16,
    pub b_y: u16,
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect {
            x: (pos.b_x * 8 + pos.x) as f32,
            y: (HEIGHT as f64 - 20.0 - pos.b_y as f64 * 8.0 + pos.y as f64) as f32,
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
        let mut total_cycle: usize = 0;
        let mut b_y = 0;
        let mut y = 0;
        while total_cycle < 341 * (HEIGHT + 20) as usize {
            let cpu_cycle = self.step();
            let ppu_cycle = self.ppu.cycle + cpu_cycle as usize;
            if ppu_cycle >= 341 / 3 {
                if y == 7 {
                    if b_y == HEIGHT / 8 - 1 {
                        break;
                    }
                    y = 0;
                    b_y += 1;
                } else {
                    y += 1;
                };
                self.ppu.cycle = self.ppu.cycle + cpu_cycle as usize - 341 / 3;
            } else {
                self.ppu.cycle += cpu_cycle as usize;
                continue;
            };
            self.ppu.cycle += cpu_cycle as usize;
            total_cycle += cpu_cycle as usize;
            for b_x in 0..WIDTH / 8 {
                for x in 0..8 {
                    self.ppu.step(x, y, b_x, b_y, &mut mesh)
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
    pub fn run(&mut self, file_stem: &str) -> GameResult {
        let (ctx, events_loop) = &mut ggez::ContextBuilder::new("nes emulator", "TaKO8Ki")
            .window_setup(ggez::conf::WindowSetup::default().title(file_stem))
            .window_mode(
                ggez::conf::WindowMode::default()
                    .dimensions(WIDTH as f32, HEIGHT as f32)
                    .resizable(true),
            )
            .build()?;
        event::run(ctx, events_loop, self)
    }
}
