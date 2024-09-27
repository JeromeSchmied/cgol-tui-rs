use cgol_tui::{app::App, *};
use ratatui::crossterm::{
    event::{self, poll, Event, KeyEventKind},
    terminal::size,
};
use ratatui::{backend::Backend, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init terminal
    let mut terminal = ratatui::try_init()?;

    // create app and run it with width and height from terminal size
    // FIXME: use render_are.area() for size determination
    let wh = size()?;
    let wh = ((wh.1 - 3) * 4).min((wh.0 / 2 - 2) * 2);
    let mut app = App::new(wh);

    let res = run_app(&mut terminal, &mut app);

    // reset terminal
    ratatui::try_restore()?;

    if let Err(err) = res {
        println!("Error: {err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let mut prev_poll_t = app.poll_t;

    loop {
        app.set_wh();

        terminal.draw(|f| ui::ui(f, app))?;

        // Wait up to `poll_t` for another event
        if poll(app.poll_t)? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    return Ok(());
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
                app.set_wh();
                app.restart();
            }
        } else {
            // Timeout expired, updating life state
            app.tick();
        }
    }

    Ok(())
}
