use conways_game_of_life_cli_rs::*;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io;

fn main() -> io::Result<()> {
    println!("{}", HELP);
    std::io::stdin().read_line(&mut String::new()).unwrap();

    enable_raw_mode()?;

    if let Err(e) = run() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()?;

    Ok(())
}
