use super::{Entity, Tile};

pub struct Level {
    width: usize,
    height: usize,
    entry: (usize, usize),
    tiles: Vec<Tile>,
    actors: Vec<Entity>,
    player: Option<Entity>,
}

impl Level {
    pub fn new(width: usize, height: usize, entry: (usize, usize)) -> Self {
        Self {
            width,
            height,
            entry,
            tiles: vec![Tile::Wall; width * height],
            actors: Vec::new(),
            player: None,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn entry(&self) -> (usize, usize) {
        self.entry
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: Tile) {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x] = tile;
    }

    pub fn actors(&self) -> &[Entity] {
        &self.actors
    }

    pub fn add_actor(&mut self, e: Entity) {
        self.actors.push(e);
    }

    pub fn player(&self) -> Option<&Entity> {
        self.player.as_ref()
    }

    pub fn player_mut(&mut self) -> Option<&mut Entity> {
        self.player.as_mut()
    }

    pub fn add_player(&mut self, e: Entity) {
        self.player = Some(e);
    }
}
