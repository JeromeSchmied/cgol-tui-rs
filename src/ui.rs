use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
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
        .constraints([Constraint::Min(4), Constraint::Length(1)])
        .split(f.size());

    // let current_shape = shapes::get(app.wh, app.i()).unwrap();

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50)])
        .split(chunks[0]);

    let cgol = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Conway's Game of Life");
    let universe = Paragraph::new(app.universe.to_string()).block(cgol);

    // let shape = Paragraph::new(current_shape.to_string()).block(
    //     Block::default()
    //         .borders(Borders::ALL)
    //         .border_type(BorderType::Rounded),
    // );
    // f.render_widget(universe, Rect::new(0, 0, app.wh() * 2 + 2, app.wh() + 2));
    f.render_widget(
        universe,
        Rect::new(
            0,
            0,
            main_chunks[0].height * 2 - 4,
            main_chunks[0].height - 1,
        ),
    );
    // f.render_widget(shape, main_chunks[1]);

    let footer = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    let current_keys_hint = Span::styled(
        "[q]uit, [r]estart, [R]eset, [n]ext, [p]revious, play[ ]pause, 'k': faster, 'j': slower",
        Style::default().fg(Color::Yellow),
    );
    // let key_notes_footer = Paragraph::new(current_keys_hint);

    let stat_style = Style::default().fg(Color::LightBlue);
    let poll_t = Span::styled(
        format!(
            "Poll time: {}",
            if let std::time::Duration::MAX = app.poll_t() {
                "max".into()
            } else {
                format!("{:.0?}", app.poll_t())
            }
        ),
        stat_style,
    );
    // let area_size = Span::styled(format!("Area size: {}", f.size()), stat_style);
    // let wh = Span::styled(format!("wh: {:?}", app.wh()), stat_style);

    let div = Span::styled(" | ", Style::default().fg(Color::White));
    let current_stats = vec![
        current_keys_hint,
        div.clone(),
        poll_t,
        // div.clone(),
        // area_size,
        // div.clone(),
        // wh,
    ];
    let footer_data = Line::from(current_stats);

    // f.render_widget(key_notes_footer, footer[0]);
    f.render_widget(footer_data, footer[0]);
}
