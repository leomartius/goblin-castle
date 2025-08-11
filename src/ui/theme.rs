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

pub enum Decoration {
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
    Horizontal,
    Vertical,
    LeftBracket,
    RightBracket,
}

pub fn box_decoration(which: Decoration) -> Cell {
    match which {
        Decoration::TopLeftCorner => Cell::new('┌', box_fg(), box_bg()),
        Decoration::TopRightCorner => Cell::new('┐', box_fg(), box_bg()),
        Decoration::BottomLeftCorner => Cell::new('└', box_fg(), box_bg()),
        Decoration::BottomRightCorner => Cell::new('┘', box_fg(), box_bg()),
        Decoration::Horizontal => Cell::new('─', box_fg(), box_bg()),
        Decoration::Vertical => Cell::new('│', box_fg(), box_bg()),
        Decoration::LeftBracket => Cell::new('┤', box_fg(), box_bg()),
        Decoration::RightBracket => Cell::new('├', box_fg(), box_bg()),
    }
}

pub fn box_fg() -> Color {
    Color::BrightWhite
}

pub fn box_bg() -> Color {
    Color::Black
}

pub fn history_fg() -> Color {
    Color::White
}

pub fn history_bg() -> Color {
    Color::Black
}
