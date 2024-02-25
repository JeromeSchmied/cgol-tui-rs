use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

/// Create Event from ch
fn ch_to_event(ch: char) -> Event {
    Event::Key(KeyCode::Char(ch).into())
}

pub fn play_pause() -> Vec<Event> {
    vec![ch_to_event(' ')]
}

pub fn slower() -> Vec<Event> {
    vec![ch_to_event('j'), Event::Key(KeyCode::Down.into())]
}
pub fn slower_big() -> Vec<Event> {
    vec![
        ch_to_event('J'),
        Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::SHIFT)),
    ]
}

pub fn faster() -> Vec<Event> {
    vec![ch_to_event('k'), Event::Key(KeyCode::Up.into())]
}
pub fn faster_big() -> Vec<Event> {
    vec![
        ch_to_event('K'),
        Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT)),
    ]
}

pub fn quit() -> Vec<Event> {
    vec![
        Event::Key(KeyCode::Esc.into()),
        ch_to_event('q'),
        Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
    ]
}

pub fn restart() -> Vec<Event> {
    vec![ch_to_event('r')]
}

pub fn reset() -> Vec<Event> {
    vec![ch_to_event('R')]
}

pub fn next() -> Vec<Event> {
    vec![ch_to_event('n')]
}
pub fn prev() -> Vec<Event> {
    vec![ch_to_event('p')]
}

pub fn bigger() -> Vec<Event> {
    vec![ch_to_event('+')]
}
pub fn smaller() -> Vec<Event> {
    vec![ch_to_event('-')]
}
