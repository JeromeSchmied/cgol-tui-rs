use cgol_tui::{app::App, *};
use crossterm::{
    event::{self, poll, Event},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let wh = size()?;
    let mut app = App::new(wh.1 - 4);
    // app.set_width(wh.0 + 3);
    // app.set_height(wh.1 - 3);
    // app.set_wh(wh.1 + 1 - 5);
    // app.set_height(38);
    // app.set_width(DEF_WH + 2);
    // app.set_height(DEF_WH + 2);

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {err:?}");
    }
    eprintln!("initial size was: {wh:?}");

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut prev_poll_t = app.poll_t();

    loop {
        let wh = size()?;
        app.set_wh(wh.1 + 1 - 5);

        terminal.draw(|f| ui::ui(f, app))?;

        // Wait up to `poll_t` for another event
        if poll(app.poll_t())? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    kmaps::QUIT => {
                        break;
                    }
                    kmaps::SLOWER => {
                        app.slower(false);
                    }
                    kmaps::FASTER => {
                        app.faster(false);
                    }
                    kmaps::PLAY_PAUSE => {
                        app.play_pause(&mut prev_poll_t);
                    }
                    kmaps::RESTART => {
                        app.restart();
                    }
                    kmaps::NEXT => {
                        app.next();
                    }
                    kmaps::PREV => {
                        app.prev();
                    }
                    kmaps::RESET => {
                        *app = app::App::default();
                    }
                    _ => {}
                }
            } else {
                let wh = size()?;
                app.set_wh(wh.1 + 1 - 5);
                app.restart();
            }
        } else {
            // Timeout expired, updating life state
            app.tick();
        }
    }

    Ok(())
}
