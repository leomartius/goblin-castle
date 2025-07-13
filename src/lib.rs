use std::io;

use console::{Console, Event};
use logic::Game;

mod console;
mod logic;

pub fn run() -> Result<(), io::Error> {
    let mut game = Game::new(78, 22);
    let mut console = Console::new(80, 24)?;

    loop {
        console.clear();
        draw_rectangle(&mut console);
        let (x, y) = game.player();
        console.set_char(x + 1, y + 1, '@');
        console.show_cursor(x + 1, y + 1);
        console.display()?;

        match get_command(&mut console)? {
            Command::Move(dx, dy) => game.move_player(dx, dy).or_else(|_| console.alert())?,
            Command::Abort => break,
        };
    }

    Ok(())
}

// very inefficient if translated directly into curses
fn draw_rectangle(console: &mut Console) {
    for x in 1..=78 {
        console.set_char(x, 0, '-');
        console.set_char(x, 23, '-');
    }
    for y in 1..=22 {
        console.set_char(0, y, '|');
        console.set_char(79, y, '|');
    }
    console.set_char(0, 0, '+');
    console.set_char(79, 0, '+');
    console.set_char(0, 23, '+');
    console.set_char(79, 23, '+');
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
