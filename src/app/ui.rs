use crate::{app::App, app::Area};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{canvas::Canvas, Block, BorderType, Borders},
    Frame,
};

/// area of a braille character
const BRAILLE: Area = Area {
    width: 2,
    height: 4,
};

pub fn ui(f: &mut Frame, app: &mut App) {
    //  _cgol_______________
    // |                    |
    // |                    |
    // |                    |
    // |____________________|
    // |____________________|
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .split(f.area());

    let cgol = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(format!("Conway's Game of Life - {} ", app.universe.name));
    // 2 blocks less: border
    let new_area = Area::new(
        (chunks[0].width - 2) * BRAILLE.width,
        (chunks[0].height - 2) * BRAILLE.height,
    );
    // apply the area change
    if app.area != new_area {
        app.area = new_area;
        app.restart();
    }
    let universe = Canvas::default()
        // .x_bounds([0., chunks[0].height as f64 * 2. - 4.])
        // .y_bounds([0., chunks[0].height as f64 * 2. - 4.])
        .paint(|ctx| ctx.draw(&app.universe))
        .block(cgol);

    f.render_widget(universe, chunks[0]);

    let footer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    let current_keys_hint =
        "[q]uit, [r]estart, [R]eset, [n]ext, [p]rev, play[ ]pause, speed: 'k' ↑, 'j' ↓".yellow();

    let poll_t = {
        if let std::time::Duration::MAX = app.poll_t {
            "paused".into()
        } else {
            format!("Poll time: {:.0?}", app.poll_t)
        }
    }
    .light_blue();

    let div = " | ".white();
    let current_stats = vec![current_keys_hint, div, poll_t];
    let footer_data = Line::from(current_stats);

    f.render_widget(footer_data, footer[0]);
}
