use std::io::{self, Stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    event::{KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{self, Print, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType::All, EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};

use super::{Buffer, Event, Key};

use super::Color as ApiColor;
use crossterm::style::Color as TermColor;

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
        cursor: Option<(usize, usize)>,
    ) -> Result<(), io::Error> {
        debug_assert!(current.width == previous.width && current.height == previous.height);
        let (mut cx, mut cy) = (usize::MAX, usize::MAX);
        let mut last_fg = ApiColor::Default;
        let mut last_bg = ApiColor::Default;
        queue!(
            self.stdout,
            cursor::Hide,
            SetForegroundColor(TermColor::Reset),
            SetBackgroundColor(TermColor::Reset)
        )?;
        for y in 0..current.height {
            for x in 0..current.width {
                let curr = current.get(x, y);
                let prev = previous.get(x, y);
                if curr != prev {
                    if (x != cx) || (y != cy) {
                        queue!(self.stdout, MoveTo(x as u16, y as u16))?;
                    }
                    if curr.fg != last_fg {
                        queue!(self.stdout, SetForegroundColor(convert_color(curr.fg)))?;
                        last_fg = curr.fg;
                    }
                    if curr.bg != last_bg {
                        queue!(self.stdout, SetBackgroundColor(convert_color(curr.bg)))?;
                        last_bg = curr.bg;
                    }
                    queue!(self.stdout, Print(curr.ch))?;
                    (cx, cy) = (x + 1, y);
                }
            }
        }
        if let Some((x, y)) = cursor {
            debug_assert!(x < current.width && y < current.height);
            queue!(self.stdout, MoveTo(x as u16, y as u16), cursor::Show)?;
        }
        self.stdout.flush()
    }

    pub fn set_title(&mut self, title: &str) -> Result<(), io::Error> {
        execute!(self.stdout, SetTitle(title))
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
                if key.modifiers == KeyModifiers::NONE {
                    let e = match key.code {
                        KeyCode::Char(ch) => Event::KeyChar(ch),
                        KeyCode::Left => Event::KeySpecial(Key::Left),
                        KeyCode::Right => Event::KeySpecial(Key::Right),
                        KeyCode::Up => Event::KeySpecial(Key::Up),
                        KeyCode::Down => Event::KeySpecial(Key::Down),
                        KeyCode::Home => Event::KeySpecial(Key::Home),
                        KeyCode::End => Event::KeySpecial(Key::End),
                        KeyCode::PageUp => Event::KeySpecial(Key::PgUp),
                        KeyCode::PageDown => Event::KeySpecial(Key::PgDn),
                        _ => continue,
                    };
                    return Ok(e);
                }
            }
        }
    }
}

fn convert_color(color: ApiColor) -> TermColor {
    match color {
        ApiColor::Default => TermColor::Reset,
        ApiColor::Black => TermColor::Black,
        ApiColor::Red => TermColor::DarkRed,
        ApiColor::Green => TermColor::DarkGreen,
        ApiColor::Yellow => TermColor::DarkYellow,
        ApiColor::Blue => TermColor::DarkBlue,
        ApiColor::Magenta => TermColor::DarkMagenta,
        ApiColor::Cyan => TermColor::DarkCyan,
        ApiColor::White => TermColor::Grey,
        ApiColor::BrightBlack => TermColor::DarkGrey,
        ApiColor::BrightRed => TermColor::Red,
        ApiColor::BrightGreen => TermColor::Green,
        ApiColor::BrightYellow => TermColor::Yellow,
        ApiColor::BrightBlue => TermColor::Blue,
        ApiColor::BrightMagenta => TermColor::Magenta,
        ApiColor::BrightCyan => TermColor::Cyan,
        ApiColor::BrightWhite => TermColor::White,
    }
}

/// Ensure that the terminal is reset when this struct is dropped.
struct AltScreen;

impl AltScreen {
    pub fn enter() -> Result<Self, io::Error> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), cursor::Hide, EnterAlternateScreen)?;
        execute!(io::stdout(), style::ResetColor, terminal::Clear(All))?;
        Ok(Self)
    }

    fn leave() -> Result<(), io::Error> {
        execute!(io::stdout(), style::ResetColor, terminal::Clear(All))?;
        execute!(io::stdout(), LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()
    }
}

impl Drop for AltScreen {
    fn drop(&mut self) {
        Self::leave().unwrap();
    }
}
