use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{canvas::Canvas, Block, BorderType, Borders},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    //  ____________________
    // |          |         |
    // |          |         |
    // |          |         |
    // |__________|_________|
    // |____________________|
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.size());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50)])
        .split(chunks[0]);

    let cgol = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Conway's Game of Life");
    // let universe = Paragraph::new(app.universe.to_string()).block(cgol);
    // let universe = Canvas::new().block(cgol);
    let universe = Canvas::default()
        // .x_bounds([0., main_chunks[0].height as f64 * 2. - 4.])
        // .y_bounds([0., main_chunks[0].height as f64 * 2. - 4.])
        .paint(|ctx| ctx.draw(&app.universe))
        .block(cgol);

    f.render_widget(universe, main_chunks[0]);

    // f.render_widget(
    //     universe,
    //     Rect::new(
    //         0,
    //         0,
    //         main_chunks[0].height * 2 - 4,
    //         main_chunks[0].height - 1,
    //     ),
    // );

    let footer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    let current_keys_hint =
        "[q]uit, [r]estart, [R]eset, [n]ext, [p]rev, play[ ]pause, 'k': faster, 'j': slower"
            .yellow();

    let poll_t = format!(
        "Poll time: {}",
        if let std::time::Duration::MAX = app.poll_t {
            "max".into()
        } else {
            format!("{:.0?}", app.poll_t)
        }
    )
    .light_blue();

    let div = " | ".white();
    let current_stats = vec![current_keys_hint, div.clone(), poll_t];
    let footer_data = Line::from(current_stats);

    f.render_widget(footer_data, footer[0]);
}
