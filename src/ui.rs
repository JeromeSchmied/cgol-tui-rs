use crate::app::{App, CurrentScreen};
use ratatui::{
    layout::{Constraint, Direction, Layout},
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
        .constraints([Constraint::Min(8), Constraint::Length(1)])
        .split(f.size());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);
    let cgol = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Conway's Game of Life");

    f.render_widget(cgol, main_chunks[0]);

    // not yet done
    let current_keys_hint = Span::styled(
        match app.current_screen {
            CurrentScreen::Main => {
                "'q': Quit, 'r': Restart, 'R': Reset, 'n': Next, 'p': Previous, '?': Help"
            }
            CurrentScreen::Help => "Esc,'q': back to main",
        },
        Style::default().fg(Color::Yellow),
    );
    let key_notes_footer = Paragraph::new(Line::from(current_keys_hint));
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .split(chunks[1]);

    f.render_widget(key_notes_footer, footer_chunks[0]);
}
