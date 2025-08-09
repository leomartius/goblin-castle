use super::theme;
use crate::{console::Console, logic::Game};

pub fn render_map(console: &mut Console, offset_x: usize, offset_y: usize, game: &Game) {
    for y in 0..game.level().height() {
        for x in 0..game.level().width() {
            if game.level().is_visible(x, y) {
                console.set_cell(
                    x + offset_x,
                    y + offset_y,
                    theme::visible_tile(&game.level().get_tile(x, y)),
                );
            } else if game.level().is_explored(x, y) {
                console.set_cell(
                    x + offset_x,
                    y + offset_y,
                    theme::explored_tile(&game.level().get_tile(x, y)),
                );
            }
        }
    }
    for e in game.level().actors() {
        if game.level().is_visible(e.x(), e.y()) {
            console.set_cell(e.x() + offset_x, e.y() + offset_y, theme::glyph(&e.glyph));
        }
    }
    let player = game.level().player().unwrap();
    console.set_cell(
        player.x() + offset_x,
        player.y() + offset_y,
        theme::glyph(&player.glyph),
    );
    console.show_cursor(player.x() + offset_x, player.y() + offset_y);
}

pub fn render_log(
    console: &mut Console,
    offset_x: usize,
    offset_y: usize,
    lines: usize,
    game: &Game,
) {
    for (n, (msg, age)) in game.log().latest(lines).enumerate() {
        console.print(
            offset_x,
            offset_y + n,
            msg,
            theme::log_message_fg(age),
            theme::log_message_bg(age),
        );
    }
}
