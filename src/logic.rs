use level::Level;

mod fov;
mod generate;
mod level;

pub struct Game {
    level: Level,
}

impl Game {
    pub fn new() -> Self {
        let mut level = generate::generate_level();
        let (x, y) = level.entry();
        let player = Entity::new(x, y, Glyph::Player);
        level.add_player(player);
        level.update_vision();
        Game { level }
    }

    pub fn move_player(&mut self, dx: i8, dy: i8) -> Result<(), ()> {
        let player = self.level.player().unwrap();
        let x = player.x() as i32 + dx as i32;
        let y = player.y() as i32 + dy as i32;
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if x < self.level.width()
                && y < self.level.height()
                && self.level.get_tile(x, y) == Tile::Floor
            {
                let player = self.level.player_mut().unwrap();
                player.set_pos(x, y);
                self.level.update_vision();
                return Ok(());
            }
        }
        Err(())
    }

    pub fn level(&self) -> &Level {
        &self.level
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall,
    Floor,
}

pub enum Glyph {
    Player,
    Goblin,
    Hobgobin,
}

pub struct Entity {
    x: u8,
    y: u8,
    pub glyph: Glyph,
}

impl Entity {
    pub fn new(x: usize, y: usize, glyph: Glyph) -> Self {
        Entity {
            x: x as u8,
            y: y as u8,
            glyph,
        }
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }

    pub fn set_pos(&mut self, x: usize, y: usize) {
        self.x = x as u8;
        self.y = y as u8;
    }
}
