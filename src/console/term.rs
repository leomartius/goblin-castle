use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    event::{KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{Buffer, Event};

pub struct Terminal {
    stdout: Stdout,
    _screen: AltScreen,
}

impl Terminal {
    pub fn new() -> Result<Self, io::Error> {
        let stdout = io::stdout();
        let _screen = AltScreen::enter()?;
        Ok(Self { stdout, _screen })
    }

    pub fn display(
        &mut self,
        current: &Buffer,
        previous: &Buffer,
        cursor: Option<(u16, u16)>,
    ) -> Result<(), io::Error> {
        debug_assert!(current.width == previous.width && current.height == previous.height);
        let (mut cx, mut cy) = (usize::MAX, usize::MAX);
        queue!(self.stdout, cursor::Hide)?;
        for y in 0..current.height {
            for x in 0..current.width {
                let curr = current.get(x, y) as char;
                let prev = previous.get(x, y) as char;
                if curr != prev {
                    if (x != cx) || (y != cy) {
                        queue!(self.stdout, MoveTo(x as u16, y as u16))?;
                    }
                    queue!(self.stdout, Print(curr))?;
                    (cx, cy) = (x + 1, y);
                }
            }
        }
        if let Some((x, y)) = cursor {
            queue!(self.stdout, MoveTo(x, y), cursor::Show)?;
        }
        self.stdout.flush()
    }

    pub fn alert(&mut self) -> Result<(), io::Error> {
        execute!(self.stdout, Print('\x07'))
    }

    pub fn read_event(&self) -> Result<Event, io::Error> {
        loop {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }
                if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                    return Ok(Event::Abort);
                }
                if let KeyCode::Char(ch) = key.code {
                    return Ok(Event::Key(ch));
                }
            }
        }
    }
}

/// Ensure that the terminal is reset when this struct is dropped.
struct AltScreen;

impl AltScreen {
    pub fn enter() -> Result<Self, io::Error> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), cursor::Hide, EnterAlternateScreen)?;
        Ok(Self)
    }

    fn leave() -> Result<(), io::Error> {
        execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()
    }
}

impl Drop for AltScreen {
    fn drop(&mut self) {
        Self::leave().unwrap();
    }
}
