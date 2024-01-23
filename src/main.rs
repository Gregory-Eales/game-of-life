//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    glam::*,
    graphics::{self, Color, Rect},
    Context, GameResult,
};

struct MainState {
    pos_x: f32,
    square: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect{x: 0.0, y: 0.0, w: 20.0, h: 20.0},
            Color::WHITE,
        )?;


        Ok(MainState { pos_x: 0.0, square})
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.square, Vec2::new(self.pos_x, 380.0));

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("game-of-life", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}