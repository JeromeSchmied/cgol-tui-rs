use wasm_game_of_life::*;

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
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

    disable_raw_mode()
}

// if not randomly generated:
//     64×64 max: 3500
//     32×32 max: 80
//     38×38 max: 190

use crossterm::{
    cursor::MoveTo,
    event::{poll, read},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::{io, time::Duration};

fn print_events() -> io::Result<()> {
    // widht and height, as they're the same
    let mut wh = 38;

    let mut i: usize = 0;
    let mut universe = get_shape(wh, i).unwrap();

    const DEFAULT_DUR: Duration = Duration::from_millis(400);
    let mut poll_t = DEFAULT_DUR;
    let mut paused = false;
    let mut prev_poll_t = DEFAULT_DUR;

    loop {
        // Wait up to 1s for another event
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
                if let Err(fig) = get_shape(wh - 1, i) {
                    eprintln!("Couldn't switch to next shape: {}\r", fig);
                    continue;
                }
                universe = get_shape(wh, i).unwrap();
            } else if kmaps::smaller().contains(&event) {
                if let Err(fig) = get_shape(wh - 1, i) {
                    eprintln!("Couldn't make smaller: {}", fig);
                } else {
                    wh -= 1;
                }
                universe = get_shape(wh, i).unwrap();
            } else if kmaps::bigger().contains(&event) {
                if let Err(fig) = get_shape(wh + 1, i) {
                    eprintln!("Couldn't make larger: {}", fig);
                } else {
                    wh += 1;
                }
                universe = get_shape(wh, i).unwrap();
            } else {
                println!("Unknown: Event::{:?}\r", event);
                std::thread::sleep(DEFAULT_DUR * 4);
            }
        } else {
            // Timeout expired, no event for 1s
            // println!(".\r");
            universe.tick();
            execute!(io::stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
            println!("{}", universe);
        }
    }

    Ok(())
}
