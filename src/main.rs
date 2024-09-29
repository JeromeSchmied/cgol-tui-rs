use app::{App, Universe};
use std::{io::Read, str::FromStr};

pub mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.contains(&"-h".into()) || args.contains(&"--help".into()) {
        println!(
            "A Conway's Game of Life viewer TUI.
            
USAGE: cgol-tui [<pattern>,...]

where <pattern> is either a .cells file, or - for stdin"
        );
        std::process::exit(1);
    }

    let piped_universe = {
        let mut univ = String::new();
        if args.len() == 1 && args[0] == "-" {
            std::io::stdin().read_to_string(&mut univ)?;
        }
        Universe::from_str(&univ)?
    };

    let mut universes = args
        .iter()
        .flat_map(std::fs::read_to_string)
        .map(|s| Universe::from_str(&s))
        .collect::<Result<Vec<_>, _>>()?;
    universes.push(piped_universe);
    let mut app = App::default().with_universes(universes);

    let mut terminal = ratatui::try_init()?;

    let res = app.run(&mut terminal);

    ratatui::try_restore()?;

    // if any error has occured while executing, print it in cooked mode
    res.inspect_err(|e| println!("error: {e:?}"))?;

    Ok(())
}
