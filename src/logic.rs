use level::Level;

mod generate;
mod level;

pub struct Game {
    level: Level,
}

impl Game {
    pub fn new() -> Self {
        let mut level = generate::generate_level();
        let player = Entity {
            x: 40,
            y: 12,
            glyph: Glyph::Player,
        };
        level.add_entity(player);
        let goblin = Entity {
            x: 30,
            y: 12,
            glyph: Glyph::Goblin,
        };
        level.add_entity(goblin);
        Game { level }
    }

    pub fn player(&self) -> &Entity {
        &self.level.entities()[0]
    }

    fn player_mut(&mut self) -> &mut Entity {
        &mut self.level.entities_mut()[0]
    }

    pub fn move_player(&mut self, dx: i8, dy: i8) -> Result<(), ()> {
        let x = self.player().x as i32 + dx as i32;
        let y = self.player().y as i32 + dy as i32;
        if x >= 0 && y >= 0 {
            let x = x as usize;
            let y = y as usize;
            if x < self.level.width()
                && y < self.level.height()
                && self.level.get_tile(x, y) == Tile::Floor
            {
                self.player_mut().x = x as u8;
                self.player_mut().y = y as u8;
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
}

pub struct Entity {
    pub x: u8,
    pub y: u8,
    pub glyph: Glyph,
}
