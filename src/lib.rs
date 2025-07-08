use std::io;

use logic::Game;

mod logic;

pub fn run() -> Result<(), io::Error> {
    let _game = Game::new();

    Ok(())
}
