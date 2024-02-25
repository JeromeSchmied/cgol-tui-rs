use conways_game_of_life_cli_rs::*;

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
                // queue!(io::stdout(), Print("Quitting..."))?;
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
            // } else if event == MouseEventKind::Down(MouseButton::Left.into()) {
            // } else if event == Event::Mouse(MouseEvent { kind, column , row , modifiers  }) {
            // } else {
            // println!("Unknown: Event::{:?}\r", event);
            } else {
                // match event {
                //     crossterm::event::Event::FocusGained => eprintln!("Focus Gained."),
                //     crossterm::event::Event::FocusLost => eprintln!("Focus Lost."),
                //     crossterm::event::Event::Key(k) => {
                //         eprintln!("Unknown key: {:?}", k);
                //     }
                //     crossterm::event::Event::Mouse(m) => {
                //         if m.kind == MouseEventKind::Up(MouseButton::Left)
                //             || m.kind == MouseEventKind::Drag(MouseButton::Left)
                //         {
                //             // eprintln!("row: {}, col: {}\r", m.row - 1, m.column - 1);
                //             std::thread::sleep(DEFAULT_DUR);
                //             if m.row > 0
                //                 && m.row <= universe.height() as u16
                //                 && m.column > 0
                //                 && m.column < universe.width() as u16
                //             {
                //                 universe.toggle_cell((m.row - 1).into(), (m.column - 1).into());
                //             }
                //             println!("{}", universe);
                //         }
                //     }
                //     crossterm::event::Event::Paste(_) => eprintln!("Paste"),
                //     crossterm::event::Event::Resize(_, _) => eprintln!("Resize"),
                // }
                eprintln!("Unknown event: {:?}", event);
            }
        } else {
            // Timeout expired, updating life state
            universe.tick();
            execute!(io::stdout(), MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
            println!("{}", universe);
        }
        // io::stdout().flush()?;
    }

    execute!(
        io::stdout(),
        LeaveAlternateScreen /*, DisableMouseCapture*/
    )?;

    Ok(())
}
