use conways_game_of_life_cli_rs::*;

const DEFAULT_DUR: Duration = Duration::from_millis(400);
const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Prints current state of Conway's Game of Life if there's no event
 - Use Esc or `q` to quit
 - `j`, `k`: decreasing, increasing speed
 - press Space to pause, play
 - hit `n` to switch to next shape
 - and now, press Enter to continue
"#;

fn main() -> io::Result<()> {
    println!("{}", HELP);
    std::io::stdin().read_line(&mut String::new()).unwrap();

    enable_raw_mode()?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()?;

    Ok(())
}

use crossterm::{
    cursor::MoveTo,
    event::{poll, read},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{
    io::{self},
    time::Duration,
};

fn print_events() -> io::Result<()> {
    execute!(
        io::stdout(),
        EnterAlternateScreen /*, EnableMouseCapture*/
    )?;

    // widht and height, as they're the same
    let mut wh = 38;

    let mut i: usize = 0;
    let mut universe = get_shape(wh, i).unwrap();

    let mut poll_t = DEFAULT_DUR;
    let mut paused = false;
    let mut prev_poll_t = DEFAULT_DUR;

    loop {
        // Wait up to `poll_t` for another event
        if poll(poll_t)? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if kmaps::quit().contains(&event) {
                println!("Quitting...\r");
                break;
            } else if kmaps::slower().contains(&event) {
                if poll_t < Duration::from_millis(40) {
                    poll_t = poll_t
                        .checked_add(Duration::from_millis(1))
                        .unwrap_or(DEFAULT_DUR);
                } else {
                    poll_t = poll_t
                        .checked_add(Duration::from_millis(10))
                        .unwrap_or(DEFAULT_DUR);
                }
                // queue!(io::stdout(), Print("Poll time is now"))?;
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::faster().contains(&event) {
                if poll_t < Duration::from_millis(40) {
                    poll_t = poll_t
                        .checked_sub(Duration::from_millis(1))
                        .unwrap_or(DEFAULT_DUR);
                } else {
                    poll_t = poll_t
                        .checked_sub(Duration::from_millis(10))
                        .unwrap_or(DEFAULT_DUR);
                }
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::slower_big().contains(&event) {
                poll_t = poll_t
                    .checked_add(Duration::from_millis(100))
                    .unwrap_or(Duration::from_millis(400));
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::faster_big().contains(&event) {
                poll_t = poll_t
                    .checked_sub(Duration::from_millis(100))
                    .unwrap_or(Duration::from_millis(400));
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::play_pause().contains(&event) {
                if paused {
                    println!("Resuming: poll() = {:?}\r", prev_poll_t);
                    poll_t = prev_poll_t;
                } else {
                    println!("Pausing...\r");
                    prev_poll_t = poll_t;
                    poll_t = Duration::MAX;
                }
                paused = !paused;
            } else if kmaps::reset().contains(&event) {
                universe = get_shape(wh, i).unwrap();
            } else if kmaps::next().contains(&event) {
                if i + 1 != SHAPES_N as usize {
                    i += 1;
                } else {
                    i = 0;
                }
                if let Ok(shape) = get_shape(wh, i) {
                    universe = shape;
                } else {
                    eprintln!("Couldn't switch to next shape\r");
                }
            } else if kmaps::smaller().contains(&event) {
                if let Ok(shape) = get_shape(wh - 1, i) {
                    universe = shape;
                    wh -= 1;
                } else {
                    eprintln!("Couldn't make smaller");
                }
            } else if kmaps::bigger().contains(&event) {
                if let Ok(shape) = get_shape(wh + 1, i) {
                    universe = shape;
                    wh += 1;
                } else {
                    eprintln!("Couldn't make larger");
                }
            } else {
                eprintln!("Unknown event: {:?}", event);
            }
        } else {
            // Timeout expired, updating life state
            universe.tick();
            execute!(io::stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
            println!("{}", universe);
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
