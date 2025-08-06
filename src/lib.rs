use std::io;

use console::{Console, Event};
use logic::Game;

mod console;
mod logic;
mod render;

pub fn run() -> Result<(), io::Error> {
    let mut console = Console::new(80, 24, "Goblin Castle")?;
    let mut game = Game::new();

    loop {
        render::render_map(&mut console, &game)?;

        match get_command(&mut console)? {
            Command::Move(dx, dy) => game.move_player(dx, dy).or_else(|_| console.alert())?,
            Command::Abort => break,
        };
    }

    Ok(())
}

pub enum Command {
    Move(i8, i8),
    Abort,
}

pub fn get_command(console: &mut Console) -> Result<Command, io::Error> {
    loop {
        let event = console.read_event()?;

        let command = match event {
            Event::Abort => Command::Abort,
            Event::Key('y') => Command::Move(-1, -1),
            Event::Key('k') => Command::Move(0, -1),
            Event::Key('u') => Command::Move(1, -1),
            Event::Key('h') => Command::Move(-1, 0),
            Event::Key('.') => Command::Move(0, 0),
            Event::Key('l') => Command::Move(1, 0),
            Event::Key('b') => Command::Move(-1, 1),
            Event::Key('j') => Command::Move(0, 1),
            Event::Key('n') => Command::Move(1, 1),
            _ => {
                console.alert()?;
                continue;
            }
        };
        return Ok(command);
    }
}
