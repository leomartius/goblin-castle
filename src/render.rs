use std::io;

use crate::{console::Console, logic::Game};

mod theme;

pub fn render_map(console: &mut Console, game: &Game) -> Result<(), io::Error> {
    console.clear();
    for y in 0..game.level().height() {
        for x in 0..game.level().width() {
            if game.level().is_visible(x, y) {
                console.set_cell(x, y, theme::visible_tile(&game.level().get_tile(x, y)));
            } else if game.level().is_explored(x, y) {
                console.set_cell(x, y, theme::explored_tile(&game.level().get_tile(x, y)));
            }
        }
    }
    for e in game.level().actors() {
        if game.level().is_visible(e.x(), e.y()) {
            console.set_cell(e.x(), e.y(), theme::glyph(&e.glyph));
        }
    }
    let player = game.level().player().unwrap();
    console.set_cell(player.x(), player.y(), theme::glyph(&player.glyph));
    console.show_cursor(player.x(), player.y());
    console.display()?;
    Ok(())
}
