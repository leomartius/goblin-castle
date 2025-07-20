use std::io;

use console::{Console, Event};
use logic::Game;

mod console;
mod logic;

pub fn run() -> Result<(), io::Error> {
    let mut console = Console::new(80, 24, "Goblin Castle")?;
    let mut game = Game::new();

    loop {
        console.clear();

        for y in 0..24 {
            for x in 0..80 {
                let ch = match game.level().get_tile(x, y) {
                    logic::Tile::Wall => '#',
                    logic::Tile::Floor => '.',
                };
                console.set_char(x, y, ch);
            }
        }

        for e in game.level().entities() {
            let ch = match e.glyph {
                logic::Glyph::Player => '@',
                logic::Glyph::Goblin => 'g',
            };
            console.set_char(e.x as usize, e.y as usize, ch);
        }
        let player = game.player();
        console.show_cursor(player.x as usize, player.y as usize);
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
