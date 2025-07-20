use super::{Entity, Tile};

pub struct Level {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
    entities: Vec<Entity>,
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![Tile::Wall; width * height],
            entities: Vec::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x] = tile;
    }

    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut [Entity] {
        &mut self.entities
    }

    pub fn add_entity(&mut self, e: Entity) {
        self.entities.push(e);
    }
}
