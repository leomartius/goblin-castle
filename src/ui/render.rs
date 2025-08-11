use crate::{console::Console, logic::Game};

use super::theme::{self, Decoration};
use super::{
    CONSOLE_HEIGHT, CONSOLE_WIDTH, LOG_LINES, LOG_OFFSET_X, LOG_OFFSET_Y, MAP_OFFSET_X,
    MAP_OFFSET_Y, POPUP_MARGIN_H, POPUP_MARGIN_V,
};

pub fn render_map(console: &mut Console, game: &Game) {
    for y in 0..game.level().height() {
        for x in 0..game.level().width() {
            if game.level().is_visible(x, y) {
                console.set_cell(
                    x + MAP_OFFSET_X,
                    y + MAP_OFFSET_Y,
                    theme::visible_tile(&game.level().get_tile(x, y)),
                );
            } else if game.level().is_explored(x, y) {
                console.set_cell(
                    x + MAP_OFFSET_X,
                    y + MAP_OFFSET_Y,
                    theme::explored_tile(&game.level().get_tile(x, y)),
                );
            }
        }
    }
    for e in game.level().actors() {
        if game.level().is_visible(e.x(), e.y()) {
            console.set_cell(
                e.x() + MAP_OFFSET_X,
                e.y() + MAP_OFFSET_Y,
                theme::glyph(&e.glyph),
            );
        }
    }
    let player = game.level().player().unwrap();
    console.set_cell(
        player.x() + MAP_OFFSET_X,
        player.y() + MAP_OFFSET_Y,
        theme::glyph(&player.glyph),
    );
    console.show_cursor(player.x() + MAP_OFFSET_X, player.y() + MAP_OFFSET_Y);
}

pub fn render_log(console: &mut Console, game: &Game) {
    for (n, (msg, age)) in game.log().latest(LOG_LINES).enumerate() {
        console.print(
            LOG_OFFSET_X,
            LOG_OFFSET_Y + n,
            msg,
            theme::log_message_fg(age),
            theme::log_message_bg(age),
        );
    }
}

pub fn render_history_box(console: &mut Console, game: &Game, scroll: usize) {
    let x0 = POPUP_MARGIN_H;
    let y0 = POPUP_MARGIN_V;
    let x1 = CONSOLE_WIDTH - POPUP_MARGIN_H - 1;
    let y1 = CONSOLE_HEIGHT - POPUP_MARGIN_V - 1;
    draw_box(console, x0, y0, x1, y1);

    let title = " Message history ";
    draw_bracketed_center(console, (x0 + x1) / 2, y0, title);
    let footer = " Up/Dn ";
    draw_bracketed_right(console, x1 - 2, y1, footer);

    let x0 = POPUP_MARGIN_H + 1;
    let y0 = POPUP_MARGIN_V + 1;
    let width = CONSOLE_WIDTH - POPUP_MARGIN_H * 2 - 2;
    let height = CONSOLE_HEIGHT - POPUP_MARGIN_V * 2 - 2;
    console.clear_rect(x0, y0, width, height);
    for (n, msg) in game.log().peek(scroll, height).enumerate() {
        let line: String = msg.chars().take(width - 2).collect();
        console.print(
            x0 + 1,
            y0 + n,
            &line,
            theme::history_fg(),
            theme::history_bg(),
        );
    }
}

fn draw_box(console: &mut Console, x0: usize, y0: usize, x1: usize, y1: usize) {
    console.set_cell(x0, y0, theme::box_decoration(Decoration::TopLeftCorner));
    console.set_cell(x1, y0, theme::box_decoration(Decoration::TopRightCorner));
    console.set_cell(x0, y1, theme::box_decoration(Decoration::BottomLeftCorner));
    console.set_cell(x1, y1, theme::box_decoration(Decoration::BottomRightCorner));
    for x in x0 + 1..=x1 - 1 {
        console.set_cell(x, y0, theme::box_decoration(Decoration::Horizontal));
        console.set_cell(x, y1, theme::box_decoration(Decoration::Horizontal));
    }
    for y in y0 + 1..=y1 - 1 {
        console.set_cell(x0, y, theme::box_decoration(Decoration::Vertical));
        console.set_cell(x1, y, theme::box_decoration(Decoration::Vertical));
    }
}

fn draw_bracketed_center(console: &mut Console, xc: usize, y: usize, text: &str) {
    let l = text.len();
    let x0 = xc - l / 2 + 1;
    console.set_cell(x0 - 1, y, theme::box_decoration(Decoration::LeftBracket));
    console.print(x0, y, text, theme::box_fg(), theme::box_bg());
    console.set_cell(x0 + l, y, theme::box_decoration(Decoration::RightBracket));
}

fn draw_bracketed_right(console: &mut Console, xr: usize, y: usize, text: &str) {
    let x0 = xr - text.len();
    console.set_cell(x0 - 1, y, theme::box_decoration(Decoration::LeftBracket));
    console.print(x0, y, text, theme::box_fg(), theme::box_bg());
    console.set_cell(xr, y, theme::box_decoration(Decoration::RightBracket));
}
