use conways_game_of_life_cli_rs::*;

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
    execute!(io::stdout(), EnterAlternateScreen)?;

    // widht and height, as they're the same
    let mut wh = DEF_WH;

    let mut i: usize = 0;
    let mut universe = get_shape(wh, i).unwrap();

    let mut poll_t = DEF_DUR;
    let mut paused = false;
    let mut prev_poll_t = poll_t;

    loop {
        // Wait up to `poll_t` for another event
        if poll(poll_t)? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if kmaps::quit().contains(&event) {
                println!("Quitting...\r");
                break;
            } else if kmaps::slower().contains(&event) {
                // if poll_t < Duration::from_millis(40) {
                //     poll_t = poll_t
                //         .checked_add(Duration::from_millis(1))
                //         .unwrap_or(DEFAULT_DUR);
                // } else {
                //     poll_t = poll_t
                //         .checked_add(Duration::from_millis(10))
                //         .unwrap_or(DEFAULT_DUR);
                // }
                if !paused {
                    slower(&mut poll_t, false);
                }
                // queue!(io::stdout(), Print("Poll time is now"))?;
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::faster().contains(&event) {
                // if poll_t < Duration::from_millis(40) {
                //     poll_t = poll_t
                //         .checked_sub(Duration::from_millis(1))
                //         .unwrap_or(DEFAULT_DUR);
                // } else {
                //     poll_t = poll_t
                //         .checked_sub(Duration::from_millis(10))
                //         .unwrap_or(DEFAULT_DUR);
                // }
                if !paused {
                    faster(&mut poll_t, false);
                }

                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::slower_big().contains(&event) {
                // poll_t = poll_t
                //     .checked_add(Duration::from_millis(100))
                //     .unwrap_or(Duration::from_millis(400));
                if !paused {
                    slower(&mut poll_t, true);
                }
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::faster_big().contains(&event) {
                // poll_t = poll_t
                //     .checked_sub(Duration::from_millis(100))
                //     .unwrap_or(Duration::from_millis(400));
                if !paused {
                    faster(&mut poll_t, true);
                }
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
            } else if kmaps::restart().contains(&event) {
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
                }
                wh += 1;
            } else if kmaps::reset().contains(&event) {
                i = 0;
                paused = false;
                wh = DEF_WH;
                poll_t = DEF_DUR;
                prev_poll_t = poll_t;
                universe = get_shape(wh, i).unwrap();
            } else {
                eprintln!("Unknown event: {event:?}\r");
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
