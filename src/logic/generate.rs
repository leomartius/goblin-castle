use std::cmp::{max, min};

use log::info;
use rand::{Rng, SeedableRng, rngs::SmallRng};

use super::{Entity, Glyph, Tile, level::Level};

struct Room {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Room {
    fn random(
        min_width: usize,
        min_height: usize,
        max_width: usize,
        max_height: usize,
        map_width: usize,
        map_height: usize,
        rng: &mut impl Rng,
    ) -> Self {
        let width = rng.random_range(min_width..=max_width);
        let height = rng.random_range(min_height..=max_height);
        let x = rng.random_range(0..=map_width - width);
        let y = rng.random_range(0..=map_height - height);
        Room {
            x0: x,
            y0: y,
            x1: x + width - 1,
            y1: y + height - 1,
        }
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x0 <= other.x1 && self.x1 >= other.x0 && self.y0 <= other.y1 && self.y1 >= other.y0
    }

    fn carve(&self, level: &mut Level) {
        for y in self.y0 + 1..=self.y1 - 1 {
            for x in self.x0 + 1..=self.x1 - 1 {
                level.set_tile(x, y, Tile::Floor);
            }
        }
    }

    fn tunnel_to(&self, other: &Room, level: &mut Level, rng: &mut impl Rng) {
        let xa = rng.random_range(self.x0 + 1..=self.x1 - 1);
        let ya = rng.random_range(self.y0 + 1..=self.y1 - 1);
        let xb = rng.random_range(other.x0 + 1..=other.x1 - 1);
        let yb = rng.random_range(other.y0 + 1..=other.y1 - 1);

        let xm;
        let ym;
        if rng.random() {
            xm = xa;
            ym = yb;
        } else {
            xm = xb;
            ym = ya;
        }

        draw_line(xa, ya, xm, ym, level);
        draw_line(xm, ym, xb, yb, level);
    }

    fn pick_xy(&self, rng: &mut impl Rng) -> (usize, usize) {
        let x = rng.random_range(self.x0 + 1..=self.x1 - 1);
        let y = rng.random_range(self.y0 + 1..=self.y1 - 1);
        (x, y)
    }
}

pub fn generate_level() -> Level {
    let width = 80;
    let height = 24;
    let mut rng = seeded_rng();
    let mut rooms: Vec<Room> = Vec::new();

    'outer: for _ in 0..40 {
        let room = Room::random(6, 6, width / 3, height / 3, width, height, &mut rng);
        for other_room in &rooms {
            if room.intersects(other_room) {
                continue 'outer;
            }
        }
        rooms.push(room);
    }

    let entry_point = rooms[0].pick_xy(&mut rng);
    let mut level = Level::new(width, height, entry_point);

    for room in &rooms {
        room.carve(&mut level);
    }

    for room1_room2 in rooms.windows(2) {
        if let [room1, room2] = room1_room2 {
            room1.tunnel_to(room2, &mut level, &mut rng);
        }
    }

    for room in &rooms {
        place_monsters(room, &mut level, &mut rng);
    }

    level
}

fn seeded_rng() -> impl Rng {
    let seed: u64 = rand::rng().random();
    info!("Level seed is 0x{seed:08X?}");
    SmallRng::seed_from_u64(seed)
}

fn draw_line(x1: usize, y1: usize, x2: usize, y2: usize, level: &mut Level) {
    debug_assert!(x1 == x2 || y1 == y2);
    if x1 == x2 {
        for y in min(y1, y2)..=max(y1, y2) {
            level.set_tile(x1, y, Tile::Floor);
        }
    } else {
        for x in min(x1, x2)..=max(x1, x2) {
            level.set_tile(x, y1, Tile::Floor);
        }
    }
}

fn place_monsters(room: &Room, level: &mut Level, rng: &mut impl Rng) {
    for _ in 0..rng.random_range(0..=2) {
        let (x, y) = room.pick_xy(rng);
        if level.actors().iter().any(|e| e.pos() == (x, y)) {
            continue;
        }
        let glyph = if rng.random_ratio(4, 5) {
            Glyph::Goblin
        } else {
            Glyph::Hobgobin
        };
        let goblin = Entity::new(x, y, glyph);
        level.add_actor(goblin);
    }
}
