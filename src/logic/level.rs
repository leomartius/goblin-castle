use super::fov::compute_fov;
use super::{Entity, Tile};

pub struct Level {
    width: usize,
    height: usize,
    entry: (usize, usize),
    tiles: Vec<Tile>,
    visible: Vec<bool>,
    explored: Vec<bool>,
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
            visible: vec![false; width * height],
            explored: vec![false; width * height],
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

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.width && y < self.height);
        self.visible[y * self.width + x]
    }

    pub fn is_explored(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.width && y < self.height);
        self.explored[y * self.width + x]
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

    pub fn update_vision(&mut self) {
        // update visibility based on the player's position
        compute_fov(
            &mut self.visible,
            self.width as i32,
            self.height as i32,
            self.player.as_ref().unwrap().x() as i32,
            self.player.as_ref().unwrap().y() as i32,
        );
        // update explored tiles based on current visibility
        for (e, v) in self.explored.iter_mut().zip(&self.visible) {
            *e = *e || *v;
        }
    }
}
