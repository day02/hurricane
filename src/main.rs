mod game;
mod engine;

use game::Game;
use engine::GraphicsEngine;

fn main() -> Result<(), String> {
    let engine = Box::new(GraphicsEngine::new());
    let game = Game::new(engine);
    game.play()?;
    Ok(())
}
