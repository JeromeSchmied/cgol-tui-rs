use wasm_game_of_life::*;

fn main() -> io::Result<()> {
    println!("{}", HELP);
    std::io::stdin().read_line(&mut String::new())?;

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

const HELP: &str = r#"Blocking poll() & non-blocking read()
 - Keyboard, mouse and terminal resize events enabled
 - Prints "." every second if there's no event
 - Hit "c" to print current cursor position
 - Use Esc or q to quit
 - j, k: decreasing, increasing poll() speed
 - press Enter to continue
"#;

fn print_events() -> io::Result<()> {
    // widht and height, as they're the same
    let mut wh = 38;
    let mut i: usize = 0;
    // let figures = vec![
    //     figures::gosper_glider_gun(),
    //     figures::featherweigth_spaceship(),
    //     figures::copperhead(),
    //     // random
    //     // stripes should be here, but not good yet
    // ];
    const DEFAULT_POLL: Duration = Duration::from_millis(400);

    // let mut universe = Universe::from_figur(wh, figures[i].clone());
    let mut universe = figurás(wh, i).unwrap();
    // let mut universe = figures::stripes(wh, wh);

    let mut stdout = io::stdout();

    let mut poll_t = DEFAULT_POLL;
    let mut paused = false;
    let mut prev_poll_t = DEFAULT_POLL;

    loop {
        // Wait up to 1s for another event
        if poll(poll_t)? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if keymaps::quit().contains(&event) {
                println!("Quitting...\r");
                break;
            } else if keymaps::slow_down().contains(&event) {
                if poll_t < Duration::from_millis(40) {
                    poll_t = poll_t
                        .checked_add(Duration::from_millis(1))
                        .unwrap_or(DEFAULT_POLL);
                } else {
                    poll_t = poll_t
                        .checked_add(Duration::from_millis(10))
                        .unwrap_or(DEFAULT_POLL);
                }

                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::speed_up().contains(&event) {
                if poll_t < Duration::from_millis(40) {
                    poll_t = poll_t
                        .checked_sub(Duration::from_millis(1))
                        .unwrap_or(DEFAULT_POLL);
                } else {
                    poll_t = poll_t
                        .checked_sub(Duration::from_millis(10))
                        .unwrap_or(DEFAULT_POLL);
                }
                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::slow_down_big().contains(&event) {
                poll_t = poll_t
                    .checked_add(Duration::from_millis(100))
                    .unwrap_or(Duration::from_millis(400));
                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::speed_up_big().contains(&event) {
                poll_t = poll_t
                    .checked_sub(Duration::from_millis(100))
                    .unwrap_or(Duration::from_millis(400));
                println!("poll time is now: {:?}\r", poll_t);
            } else if keymaps::play_pause().contains(&event) {
                if paused {
                    println!("Resuming: poll() = {:?}\r", prev_poll_t);
                    poll_t = prev_poll_t;
                } else {
                    println!("Pausing...\r");
                    prev_poll_t = poll_t;
                    poll_t = Duration::MAX;
                }
                paused = !paused;
            } else if keymaps::reset().contains(&event) {
                universe = figurás(wh, i).unwrap();
            } else if keymaps::next().contains(&event) {
                if i + 1 != FIG_NUM as usize {
                    i += 1;
                } else {
                    i = 0;
                }
                // universe = Universe::from_figur(wh, figures[i].clone());
                universe = figurás(wh, i).unwrap();
            } else if keymaps::smaller().contains(&event) {
                if let Err(fig) = figurás(wh - 1, i) {
                    eprintln!("Couldn't make smaller: {}", fig);
                } else {
                    wh -= 1;
                }
                universe = figurás(wh, i).unwrap();
                // if i == figures.len() {
                //     universe = figures::rand(wh, wh);
                // } else {
                //     universe = Universe::from_figur(wh, figures[i].clone());
                // }
            } else if keymaps::bigger().contains(&event) {
                if let Err(fig) = figurás(wh + 1, i) {
                    eprintln!("Couldn't make larger: {}", fig);
                } else {
                    wh += 1;
                }
                universe = figurás(wh, i).unwrap();
                // if i == figures.len() {
                //     universe = figures::rand(wh, wh);
                // } else {
                //     universe = Universe::from_figur(wh, figures[i].clone());
                // }
            } else {
                println!("Unknown: Event::{:?}\r", event);
            }
        } else {
            // Timeout expired, no event for 1s
            // println!(".\r");
            universe.tick();
            execute!(stdout, MoveTo(0, 0), Clear(ClearType::FromCursorDown))?;
            // execute!(stdout, Clear(ClearType::All))?;
            println!("{}", universe);
        }
    }

    Ok(())
}
