use ratatui::crossterm::event::KeyCode;

pub const PLAY_PAUSE: KeyCode = KeyCode::Char(' ');

pub const SLOWER: KeyCode = KeyCode::Char('j');
pub const SLOWER_BIG: KeyCode = KeyCode::Char('J');

pub const FASTER: KeyCode = KeyCode::Char('k');
pub const FASTER_BIG: KeyCode = KeyCode::Char('K');

pub const QUIT: KeyCode = KeyCode::Char('q');

pub const RESTART: KeyCode = KeyCode::Char('r');

pub const RESET: KeyCode = KeyCode::Char('R');

pub const NEXT: KeyCode = KeyCode::Char('n');
pub const PREV: KeyCode = KeyCode::Char('p');

pub const HELP: KeyCode = KeyCode::Char('?');
