//! cargo run --example basic
#![allow(clippy::unnecessary_wraps)]

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult, ContextBuilder};
use glam::*;

struct GameState {
    pos_x: f32,
}

impl GameState {
    fn new() -> GameState {
        Self {
            pos_x: 0.0
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            2.0,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (Vec2::new(self.pos_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");
    let state = GameState::new();
    event::run(ctx, event_loop, state)
}