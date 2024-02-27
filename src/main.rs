use crate::app::App;
use conways_game_of_life_cli_rs::*;
use crossterm::{
    cursor::MoveTo,
    event::{poll, read},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", HELP);
    io::stdin().read_line(&mut String::new())?;

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
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
                *app = app::App::default();
            } else {
                eprintln!("Unknown event: {event:?}\r");
            }
        } else {
            // Timeout expired, updating life state
            execute!(io::stdout(), MoveTo(0, 0), Clear(ClearType::All))?;
            app.render_universe();
            app.tick();
        }
    }

    Ok(())
}
