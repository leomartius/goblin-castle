use level::Level;

mod level;

pub struct Game {
    width: u16,
    height: u16,
    player: (u16, u16),
    level: Level,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        let mut level = Level::new(width.into(), height.into());
        for y in 7..17 {
            for x in 25..55 {
                level.set_tile(x, y, Tile::Floor);
            }
        }
        Game {
            width,
            height,
            player: (width / 2, height / 2),
            level,
        }
    }

    pub fn player(&self) -> (u16, u16) {
        self.player
    }

    pub fn move_player(&mut self, dx: i8, dy: i8) -> Result<(), ()> {
        let x = self.player.0 as i32 + dx as i32;
        let y = self.player.1 as i32 + dy as i32;
        if 0 <= x && x < self.width.into() && 0 <= y && y < self.height.into() {
            let x = x as usize;
            let y = y as usize;
            if self.level.get_tile(x, y) == Tile::Floor {
                self.player = (x as u16, y as u16);
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
