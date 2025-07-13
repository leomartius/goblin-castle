pub struct Game {
    width: u16,
    height: u16,
    player: (u16, u16),
}

impl Game {
    pub fn new(width: u16, height: u16) -> Self {
        Game {
            width,
            height,
            player: (width / 2, height / 2),
        }
    }

    pub fn player(&self) -> (u16, u16) {
        self.player
    }

    pub fn move_player(&mut self, dx: i8, dy: i8) -> Result<(), ()> {
        let x = self.player.0 as i32 + dx as i32;
        let y = self.player.1 as i32 + dy as i32;
        if 0 <= x && x < self.width.into() && 0 <= y && y < self.height.into() {
            self.player = (x as u16, y as u16);
            return Ok(());
        }
        Err(())
    }
}
