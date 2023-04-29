use std::ptr::null;

use ggez::{
    self,
    event::{self, EventHandler},
    graphics::{self, Color, DrawParam, Drawable, Rect},
    Context, ContextBuilder, GameResult,
};

/// main game struct to hold the game state
struct PingPongGame {}

impl PingPongGame {
    /// Main game instance
    pub fn new(_ctx: &mut Context) -> PingPongGame {
        PingPongGame {}
    }
}

impl EventHandler for PingPongGame {
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas_1 = graphics::Canvas::from_frame(ctx, Color::from_rgba(255, 204, 96, 1));

        // rectangular shape
        let rect = graphics::Rect::new(100.0, 100.0, 300.0, 300.0);
        let rect_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::WHITE);
        canvas_1.finish(ctx);
        Ok(())
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        return Ok(());
    }
}

/// creates new game instance
fn create_game() {
    let game_name = "Ping Pong Game";
    let author = "Ashraf Sada";
    let builder = ContextBuilder::new(game_name, author);
    let (mut ctx, event_loop) = builder.build().expect("Context failed to create!");
    ctx.gfx.set_window_title(game_name);
    let pong_game = PingPongGame::new(&mut ctx);
    event::run(ctx, event_loop, pong_game);
}

fn main() {
    println!("Ping Pong Game");

    create_game();
}
