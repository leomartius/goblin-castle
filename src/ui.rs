use crate::console::{Console, Event};
use crate::logic::Game;

pub mod input;
pub mod render;
pub mod scenes;
mod theme;

pub const CONSOLE_WIDTH: usize = 80;
pub const CONSOLE_HEIGHT: usize = 43;

pub const MAP_OFFSET_X: usize = 0;
pub const MAP_OFFSET_Y: usize = 4;

pub const LOG_OFFSET_X: usize = 0;
pub const LOG_OFFSET_Y: usize = 0;
pub const LOG_LINES: usize = 4;

pub const POPUP_MARGIN_H: usize = 6;
pub const POPUP_MARGIN_V: usize = 3;

pub enum Command {
    Move(i8, i8),
    History,
    Scroll(i8),
}

pub enum Transition {
    /// Event was handled, continue with the same scene.
    Okay,
    /// Unexpected event, show an alert and continue with the same scene.
    Beep,
    /// Event was handled, switch to a different scene.
    Switch(Box<dyn Scene>),
    /// Switch to a different scene and push it onto the stack.
    Push(Box<dyn Scene>),
    /// Return to the previous scene.
    Pop,
}

pub trait Scene {
    /// Draw one full frame of this scene.
    fn render(&self, game: &Game, console: &mut Console);

    /// Handle one event and decide the next state.
    fn handle_event(&mut self, game: &mut Game, event: Event) -> Transition;
}
