use crate::console::{Color, Console, Event};
use crate::logic::Game;

use super::{CONSOLE_HEIGHT, Command, POPUP_MARGIN_V, Scene, Transition, input, render};

pub struct StartScreen;

impl Scene for StartScreen {
    fn render(&self, _game: &Game, console: &mut Console) {
        let greeting = "Press any key to start...";
        console.print(27, 21, greeting, Color::Default, Color::Default);
        console.show_cursor(52, 21);
    }

    fn handle_event(&mut self, _game: &mut Game, _event: Event) -> Transition {
        Transition::Switch(Box::new(PlayScreen))
    }
}

pub struct PlayScreen;

impl Scene for PlayScreen {
    fn render(&self, game: &Game, console: &mut Console) {
        render::render_map(console, game);
        render::render_log(console, game);
    }

    fn handle_event(&mut self, game: &mut Game, event: Event) -> Transition {
        match input::map_play_command(event) {
            Some(command) => match command {
                Command::Move(dx, dy) => match game.move_player(dx, dy) {
                    Ok(_) => Transition::Okay,
                    Err(_) => Transition::Beep,
                },
                Command::History => Transition::Push(Box::new(HistoryPopup::new())),
                _ => unreachable!(),
            },
            None => Transition::Beep,
        }
    }
}

pub struct HistoryPopup {
    from_bottom: usize,
}

impl HistoryPopup {
    fn new() -> Self {
        Self { from_bottom: 0 }
    }
}

impl Scene for HistoryPopup {
    fn render(&self, game: &Game, console: &mut Console) {
        console.hide_cursor();
        console.dim();
        let win_height = CONSOLE_HEIGHT - POPUP_MARGIN_V * 2 - 2;
        let log_length = game.log().len();
        let scroll = log_length.saturating_sub(win_height + self.from_bottom);
        render::render_history_box(console, game, scroll);
    }

    fn handle_event(&mut self, game: &mut Game, event: Event) -> Transition {
        let win_height = CONSOLE_HEIGHT - POPUP_MARGIN_V * 2 - 2;
        let log_length = game.log().len();
        let max_scroll = log_length.saturating_sub(win_height);

        let curr = self.from_bottom as i32;
        let max = max_scroll as i32;
        match input::map_scroll_command(event) {
            Some(command) => {
                let next = match command {
                    Command::Scroll(i8::MIN) => max,
                    Command::Scroll(i8::MAX) => 0,
                    Command::Scroll(delta) => (curr - delta as i32).clamp(0, max),
                    _ => unreachable!(),
                };
                self.from_bottom = next as usize;
                Transition::Okay
            }
            None => Transition::Pop,
        }
    }
}
