use std::{io, mem};

mod term;

pub struct Console {
    terminal: term::Terminal,
    front: Buffer,
    back: Buffer,
    cursor: Option<(usize, usize)>,
}

impl Console {
    pub fn new(width: usize, height: usize, title: &str) -> Result<Self, io::Error> {
        let mut terminal = term::Terminal::new()?;
        terminal.set_title(title)?;
        Ok(Console {
            terminal,
            front: Buffer::new(width, height),
            back: Buffer::new(width, height),
            cursor: None,
        })
    }

    pub fn clear(&mut self) {
        self.back.clear();
        self.cursor = None;
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.back.set(x, y, cell);
    }

    pub fn print(&mut self, x0: usize, y0: usize, text: &str, fg: Color, bg: Color) {
        for (dx, ch) in text.chars().enumerate() {
            if x0 + dx >= self.back.width {
                break;
            }
            self.back.set(x0 + dx, y0, Cell { ch, fg, bg });
        }
    }

    pub fn show_cursor(&mut self, x: usize, y: usize) {
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

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    Default,
    // basic colors
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    // bright colors
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    ch: char,
    fg: Color,
    bg: Color,
}

impl Cell {
    pub fn new(ch: char, fg: Color, bg: Color) -> Cell {
        Cell { ch, fg, bg }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            ch: '\x20',
            fg: Color::Default,
            bg: Color::Default,
        }
    }
}

struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Buffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
        }
    }

    fn clear(&mut self) {
        self.cells.fill(Cell::default());
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        debug_assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        debug_assert!(x < self.width && y < self.height);
        self.cells[y * self.width + x] = cell;
    }
}

pub enum Event {
    Key(char),
    Abort,
}
