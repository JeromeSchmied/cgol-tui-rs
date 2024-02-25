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
    event::{poll, read},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io;

fn print_events() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;

    let mut app = App::default();

    // widht and height, as they're the same
    // let mut wh = DEF_WH;

    // let mut i: usize = 0;
    // let mut universe = shapes::get(wh, i).unwrap();

    // let mut poll_t = DEF_DUR;
    // let mut paused = false;
    let mut prev_poll_t = app.poll_t();

    loop {
        // Wait up to `poll_t` for another event
        if poll(app.poll_t())? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if kmaps::quit().contains(&event) {
                println!("Quitting...\r");
                break;
            } else if kmaps::slower().contains(&event) {
                if !app.paused() {
                    app.slower(false);
                }
                println!("poll time is now: {:?}\r", app.poll_t());
            } else if kmaps::faster().contains(&event) {
                if !app.paused() {
                    app.faster(false);
                }

                println!("poll time is now: {:?}\r", app.poll_t());
            } else if kmaps::slower_big().contains(&event) {
                if !app.paused() {
                    app.slower(true);
                }
                println!("poll time is now: {:?}\r", app.poll_t());
            } else if kmaps::faster_big().contains(&event) {
                if !app.paused() {
                    app.faster(true);
                }
                println!("poll time is now: {:?}\r", app.poll_t());
            } else if kmaps::play_pause().contains(&event) {
                app.play_pause(&mut prev_poll_t);
            } else if kmaps::restart().contains(&event) {
                app.restart();
            } else if kmaps::next().contains(&event) {
                app.next();
            } else if kmaps::prev().contains(&event) {
                app.prev();
            } else if kmaps::smaller().contains(&event) {
                app.smaller();
            } else if kmaps::bigger().contains(&event) {
                app.bigger();
            } else if kmaps::reset().contains(&event) {
                app = App::default();
            } else {
                eprintln!("Unknown event: {event:?}\r");
            }
        } else {
            // Timeout expired, updating life state
            execute!(io::stdout(), Clear(ClearType::All))?;
            println!("{}", app.render_universe());
            app.tick();
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}
