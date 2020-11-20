use crate::nes::Nes;
use ggez::event;
use ggez::{Context, GameResult};

const WIDTH: u16 = 256;
const HEIGHT: u8 = 240;
const OVERLOAD: u8 = 12;

impl event::EventHandler for Nes {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.step();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
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
