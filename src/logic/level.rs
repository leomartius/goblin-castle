use super::Tile;

pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::Wall; width * height],
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x] = tile;
    }
}
