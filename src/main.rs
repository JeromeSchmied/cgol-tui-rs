use wasm_game_of_life::*;

fn main() -> io::Result<()> {
    println!("{}", HELP);
    std::io::stdin().read_line(&mut String::new())?;

    enable_raw_mode()?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()
}

// if not randomly generated:
//     64×64 max: 3500
//     32×32 max: 80
//     38×38 max: 190

use crossterm::{
    cursor::MoveTo,
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{io, time::Duration};

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc or q to quit
 - j, k: decreasing, increasing poll() speed
 - press Enter to continue
"#;

fn print_events() -> io::Result<()> {
    const DEFAULT_POLL: Duration = Duration::from_millis(400);

    let (mut width, mut height) = (38, 38);
    let mut is_figur = true;
    let figur = figures::gosper_glider_gun();
    let mut universe = Universe::from_figur(width, height, figur.clone());

    let mut stdout = io::stdout();

    let mut poll_t = DEFAULT_POLL;
    let mut paused = false;
    let mut prev_poll_t = DEFAULT_POLL;

    loop {
        // Wait up to 1s for another event
        if poll(poll_t)? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if keymaps::quit().contains(&event) {
                println!("Quitting...\r");
                break;
            } else if keymaps::slow_down().contains(&event) {
                if poll_t < Duration::from_millis(100) {
                    poll_t = poll_t
                        .checked_add(Duration::from_millis(1))
                        .unwrap_or(DEFAULT_POLL);
                } else {
                    poll_t = poll_t
                        .checked_add(Duration::from_millis(10))
                        .unwrap_or(DEFAULT_POLL);
                }

                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::speed_up().contains(&event) {
                if poll_t < Duration::from_millis(100) {
                    poll_t = poll_t
                        .checked_sub(Duration::from_millis(1))
                        .unwrap_or(DEFAULT_POLL);
                } else {
                    poll_t = poll_t
                        .checked_sub(Duration::from_millis(10))
                        .unwrap_or(DEFAULT_POLL);
                }
                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::slow_down_big().contains(&event) {
                poll_t = poll_t
                    .checked_add(Duration::from_millis(100))
                    .unwrap_or(Duration::from_millis(400));
                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::speed_up_big().contains(&event) {
                poll_t = poll_t
                    .checked_sub(Duration::from_millis(100))
                    .unwrap_or(Duration::from_millis(400));
                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::play_pause().contains(&event) {
                if paused {
                    println!("Resuming: poll() = {:?}\r", prev_poll_t);
                    poll_t = prev_poll_t;
                } else {
                    println!("Pausing...\r");
                    prev_poll_t = poll_t;
                    poll_t = Duration::MAX;
                }
                paused = !paused;
            } else if keymaps::reset().contains(&event) {
                if is_figur {
                    universe = Universe::from_figur(width, height, figur.clone());
                } else {
                    universe = Universe::new_rand(width, height);
                }
            } else if keymaps::next().contains(&event) {
                is_figur = !is_figur;
                universe = Universe::from_figur(width, height, figur.clone());
            } else if keymaps::smaller().contains(&event) {
                height -= 1;
                width -= 1;
                universe = Universe::from_figur(width, height, figur.clone());
            } else if keymaps::bigger().contains(&event) {
                height += 1;
                width += 1;
                universe = Universe::from_figur(width, height, figur.clone());
            } else {
                println!("Unknown: Event::{:?}\r", event);
            }
        } else {
            // Timeout expired, no event for 1s
            // println!(".\r");
            universe.tick();
            execute!(stdout, MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
            // execute!(stdout, Clear(ClearType::All))?;
            println!("{}", universe);
        }
    }

    Ok(())
}

mod keymaps {
    use super::*;

    pub fn play_pause() -> Vec<Event> {
        vec![Event::Key(KeyCode::Char(' ').into())]
    }

    pub fn speed_up_big() -> Vec<Event> {
        vec![
            Event::Key(KeyCode::Char('J').into()),
            Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::SHIFT)),
        ]
    }
    pub fn speed_up() -> Vec<Event> {
        vec![
            Event::Key(KeyCode::Char('j').into()),
            Event::Key(KeyCode::Down.into()),
        ]
    }

    pub fn slow_down_big() -> Vec<Event> {
        vec![
            Event::Key(KeyCode::Char('K').into()),
            Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT)),
        ]
    }
    pub fn slow_down() -> Vec<Event> {
        vec![
            Event::Key(KeyCode::Char('k').into()),
            Event::Key(KeyCode::Up.into()),
        ]
    }

    pub fn quit() -> Vec<Event> {
        vec![
            Event::Key(KeyCode::Esc.into()),
            Event::Key(KeyCode::Char('q').into()),
            Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
        ]
    }

    pub fn reset() -> Vec<Event> {
        vec![Event::Key(KeyCode::Char('R').into())]
    }

    pub fn next() -> Vec<Event> {
        vec![Event::Key(KeyCode::Char('n').into())]
    }

    pub fn bigger() -> Vec<Event> {
        vec![Event::Key(KeyCode::Char('+').into())]
    }
    pub fn smaller() -> Vec<Event> {
        vec![Event::Key(KeyCode::Char('-').into())]
    }
}
