use std::{io, mem};

mod term;

pub struct Console {
    terminal: term::Terminal,
    front: Buffer,
    back: Buffer,
    cursor: Option<(u16, u16)>,
}

impl Console {
    pub fn new(width: u16, height: u16) -> Result<Self, io::Error> {
        Ok(Console {
            terminal: term::Terminal::new()?,
            front: Buffer::new(width.into(), height.into()),
            back: Buffer::new(width.into(), height.into()),
            cursor: None,
        })
    }

    pub fn clear(&mut self) {
        self.back.clear();
        self.cursor = None;
    }

    pub fn set_char(&mut self, x: u16, y: u16, ch: char) {
        self.back.set(x.into(), y.into(), ch as u8);
    }

    pub fn show_cursor(&mut self, x: u16, y: u16) {
        self.cursor = Some((x, y));
    }

    pub fn display(&mut self) -> Result<(), io::Error> {
        mem::swap(&mut self.back, &mut self.front);
        self.terminal.display(&self.front, &self.back, self.cursor)
    }

    pub fn alert(&mut self) -> Result<(), io::Error> {
        self.terminal.alert()
    }

    pub fn read_event(&self) -> Result<Event, io::Error> {
        self.terminal.read_event()
    }
}

struct Buffer {
    width: usize,
    height: usize,
    tiles: Vec<u8>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![0x20; width * height],
        }
    }

    fn clear(&mut self) {
        self.tiles.fill(0x20);
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, tile: u8) {
        debug_assert!(x < self.width && y < self.height);
        self.tiles[y * self.width + x] = tile;
    }
}

pub enum Event {
    Key(char),
    Abort,
}
