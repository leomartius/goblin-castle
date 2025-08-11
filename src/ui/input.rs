use crate::console::{Event, Key};

use super::Command;

pub fn map_play_command(event: Event) -> Option<Command> {
    let command = match event {
        Event::KeyChar('y') => Command::Move(-1, -1),
        Event::KeyChar('k') => Command::Move(0, -1),
        Event::KeyChar('u') => Command::Move(1, -1),
        Event::KeyChar('h') => Command::Move(-1, 0),
        Event::KeyChar('l') => Command::Move(1, 0),
        Event::KeyChar('b') => Command::Move(-1, 1),
        Event::KeyChar('j') => Command::Move(0, 1),
        Event::KeyChar('n') => Command::Move(1, 1),
        Event::KeySpecial(Key::Home) => Command::Move(-1, -1),
        Event::KeySpecial(Key::Up) => Command::Move(0, -1),
        Event::KeySpecial(Key::PgUp) => Command::Move(1, -1),
        Event::KeySpecial(Key::Left) => Command::Move(-1, 0),
        Event::KeySpecial(Key::Right) => Command::Move(1, 0),
        Event::KeySpecial(Key::End) => Command::Move(-1, 1),
        Event::KeySpecial(Key::Down) => Command::Move(0, 1),
        Event::KeySpecial(Key::PgDn) => Command::Move(1, 1),
        Event::KeyChar('.') => Command::Move(0, 0),
        Event::KeyChar('m') => Command::History,
        _ => return None,
    };
    Some(command)
}

pub fn map_scroll_command(event: Event) -> Option<Command> {
    let command = match event {
        Event::KeySpecial(Key::Home) => Command::Scroll(i8::MIN),
        Event::KeySpecial(Key::PgUp) => Command::Scroll(-10),
        Event::KeySpecial(Key::Up) => Command::Scroll(-1),
        Event::KeySpecial(Key::Down) => Command::Scroll(1),
        Event::KeySpecial(Key::PgDn) => Command::Scroll(10),
        Event::KeySpecial(Key::End) => Command::Scroll(i8::MAX),
        _ => return None,
    };
    Some(command)
}
