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
        for b_y in 0..HEIGHT / 8 {
            for b_x in 0..WIDTH / 8 {
                for y in 0..8 {
                    for x in 0..8 {
                        self.build_background(x, y, b_x, b_y, &mut mesh)
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
