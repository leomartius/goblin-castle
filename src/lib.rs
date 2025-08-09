use std::io;

use console::{Console, Event};
use logic::Game;
use ui::render;

mod console;
mod logic;
mod ui;

pub fn run() -> Result<(), io::Error> {
    let mut console = Console::new(ui::CONSOLE_WIDTH, ui::CONSOLE_HEIGHT, "Goblin Castle")?;
    let mut game = Game::new();

    loop {
        console.clear();
        render::render_map(&mut console, ui::MAP_OFFSET_X, ui::MAP_OFFSET_Y, &game);
        render::render_log(
            &mut console,
            ui::LOG_OFFSET_X,
            ui::LOG_OFFSET_Y,
            ui::LOG_LINES,
            &game,
        );
        console.display()?;

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
