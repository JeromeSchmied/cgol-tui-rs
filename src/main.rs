use cgol_tui::{app::App, *};
use ratatui::crossterm::event::{self, poll, Event, KeyEventKind};
use ratatui::{backend::Backend, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up logger
    fern::Dispatch::new()
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // Output to stdout, files, and other Dispatch configurations
        .chain(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(".cgoltui.log")?,
        )
        // Apply globally
        .apply()?;

    // init terminal
    let mut terminal = ratatui::try_init()?;

    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);

    // reset terminal
    ratatui::try_restore()?;

    // if any error has occured while executing, print it in cooked mode
    res.inspect_err(|e| println!("error: {e:?}"))?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut prev_poll_t = app.poll_t;

    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        // Wait up to `poll_t` for another event
        if poll(app.poll_t)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    kmaps::QUIT => break,
                    kmaps::SLOWER => app.slower(false),
                    kmaps::FASTER => app.faster(false),
                    kmaps::PLAY_PAUSE => app.play_pause(&mut prev_poll_t),
                    kmaps::RESTART => app.restart(),
                    kmaps::NEXT => app.next(),
                    kmaps::PREV => app.prev(),
                    kmaps::RESET => *app = App::default(),
                    _ => {}
                }
            } else {
                // resize and restart
                app.restart();
            }
        } else {
            // Timeout expired, updating life state
            app.tick();
        }
    }

    Ok(())
}
