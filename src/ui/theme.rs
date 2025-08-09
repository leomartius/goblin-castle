use crate::{
    console::{Cell, Color},
    logic::{Glyph, Tile},
};

pub fn glyph(glyph: &Glyph) -> Cell {
    match glyph {
        Glyph::Player => Cell::new('@', Color::BrightWhite, Color::Black),
        Glyph::Goblin => Cell::new('g', Color::BrightRed, Color::Black),
        Glyph::Hobgobin => Cell::new('H', Color::BrightRed, Color::Black),
    }
}

pub fn visible_tile(tile: &Tile) -> Cell {
    match tile {
        Tile::Wall => Cell::new('#', Color::BrightWhite, Color::Black),
        Tile::Floor => Cell::new('.', Color::BrightWhite, Color::Black),
    }
}

pub fn explored_tile(tile: &Tile) -> Cell {
    match tile {
        Tile::Wall => Cell::new('#', Color::BrightBlack, Color::Black),
        Tile::Floor => Cell::new('.', Color::BrightBlack, Color::Black),
    }
}

pub fn log_message_fg(age: u64) -> Color {
    match age {
        0 => Color::BrightWhite,
        1 => Color::White,
        _ => Color::BrightBlack,
    }
}

pub fn log_message_bg(_age: u64) -> Color {
    Color::Black
}
