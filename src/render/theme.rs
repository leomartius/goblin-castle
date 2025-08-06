use crate::logic::{Glyph, Tile};

pub fn glyph(glyph: &Glyph) -> char {
    match glyph {
        Glyph::Player => '@',
        Glyph::Goblin => 'g',
        Glyph::Hobgobin => 'H',
    }
}

pub fn visible_tile(tile: &Tile) -> char {
    match tile {
        Tile::Wall => '#',
        Tile::Floor => '.',
    }
}

pub fn explored_tile(tile: &Tile) -> char {
    match tile {
        Tile::Wall => '#',
        Tile::Floor => '.',
    }
}
