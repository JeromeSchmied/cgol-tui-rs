use crate::app::App;
use conways_game_of_life_cli_rs::{app::CurrentScreen, *};
use crossterm::{
    event::{self, poll, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
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
        terminal.draw(|f| ui::ui(f, app))?;

        // Wait up to `poll_t` for another event
        if poll(app.poll_t())? {
            if let Event::Key(key) = event::read()? {
                match app.current_screen {
                    CurrentScreen::Main => match key.code {
                        kmaps::QUIT => {
                            eprintln!("Quitting...\r");
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
                        kmaps::HELP => {
                            app.current_screen = CurrentScreen::Help;
                        }
                        _ => {}
                    },
                    CurrentScreen::Help => match key.code {
                        kmaps::HELP | kmaps::QUIT | KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        _ => {}
                    },
                }
            }
        } else {
            // Timeout expired, updating life state
            app.tick();
        }
    }

    Ok(())
}
