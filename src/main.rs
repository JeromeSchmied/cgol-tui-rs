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
    let mut universe = shapes::get(wh, i).unwrap();

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
                if !paused {
                    slower(&mut poll_t, false);
                }
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::faster().contains(&event) {
                if !paused {
                    faster(&mut poll_t, false);
                }

                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::slower_big().contains(&event) {
                if !paused {
                    slower(&mut poll_t, true);
                }
                println!("poll time is now: {:?}\r", poll_t);
            } else if kmaps::faster_big().contains(&event) {
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
                universe = shapes::get(wh, i).unwrap();
            } else if kmaps::next().contains(&event) {
                next(&mut i, wh, &mut universe);
            } else if kmaps::prev().contains(&event) {
                prev(&mut i, wh, &mut universe);
            } else if kmaps::smaller().contains(&event) {
                if let Ok(shape) = shapes::get(wh - 1, i) {
                    universe = shape;
                    wh -= 1;
                } else {
                    eprintln!("Couldn't make smaller");
                }
            } else if kmaps::bigger().contains(&event) {
                if let Ok(shape) = shapes::get(wh + 1, i) {
                    universe = shape;
                }
                wh += 1;
            } else if kmaps::reset().contains(&event) {
                i = 0;
                paused = false;
                wh = DEF_WH;
                poll_t = DEF_DUR;
                prev_poll_t = poll_t;
                universe = shapes::get(wh, i).unwrap();
            } else {
                eprintln!("Unknown event: {event:?}\r");
            }
        } else {
            // Timeout expired, updating life state
            execute!(io::stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
            println!("{}", universe);
            universe.tick();
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
