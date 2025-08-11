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

    pub fn clear_rect(&mut self, x: usize, y: usize, width: usize, height: usize) {
        self.back.fill_rect(x, y, width, height, Cell::default());
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

    pub fn dim(&mut self) {
        self.back
            .apply(|cell| *cell = Cell::new(cell.ch, cell.fg.to_dim(), cell.bg.to_dim()));
    }

    pub fn show_cursor(&mut self, x: usize, y: usize) {
        self.cursor = Some((x, y));
    }

    pub fn hide_cursor(&mut self) {
        self.cursor = None;
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

#[allow(dead_code)]
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

impl Color {
    fn to_dim(self) -> Self {
        match self {
            // light/dark pairs
            Color::BrightRed => Color::Red,
            Color::BrightGreen => Color::Green,
            Color::BrightYellow => Color::Yellow,
            Color::BrightBlue => Color::Blue,
            Color::BrightMagenta => Color::Magenta,
            Color::BrightCyan => Color::Cyan,
            // grayscale
            Color::White => Color::BrightBlack,
            Color::BrightWhite => Color::BrightBlack,
            // unchanged
            _ => self,
        }
    }
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

    fn fill_rect(&mut self, x0: usize, y0: usize, width: usize, height: usize, value: Cell) {
        debug_assert!(x0 + width <= self.width && y0 + height <= self.height);
        for y in y0..y0 + height {
            self.cells[y * self.width + x0..y * self.width + x0 + width].fill(value);
        }
    }

    fn apply<F>(&mut self, transform: F)
    where
        F: FnMut(&mut Cell),
    {
        self.cells.iter_mut().for_each(transform);
    }
}

pub enum Event {
    Abort,
    KeyChar(char),
    KeySpecial(Key),
}

pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PgUp,
    PgDn,
}
