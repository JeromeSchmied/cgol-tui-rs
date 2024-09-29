use app::{App, Universe};
use std::io::Read;

pub mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // set up logger
    fern::Dispatch::new()
        // Add blanket level filter
        // TODO: cli -v^n, 0 < n < 5
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

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    log::debug!("args: {args:?}");
    if args.contains(&"-h".into()) || args.contains(&"--help".into()) {
        println!(
            "A Conway's Game of Life viewer TUI.
            
USAGE: cgol-tui [<pattern>,...]

where <pattern> is either a .cells file, or - for stdin"
        );
        std::process::exit(1);
    }

    let piped_universes = {
        let mut univ = String::new();
        if args.len() == 1 && args[0] == "-" {
            std::io::stdin().read_to_string(&mut univ)?;
        }

        if univ.is_empty() {
            vec![]
        } else {
            vec![Universe::from_str(univ)]
        }
    };

    let universes = args
        .iter()
        .flat_map(std::fs::read_to_string)
        .map(Universe::from_str)
        .collect::<Vec<_>>();
    let mut app = App::default().with_universes([universes, piped_universes].concat());

    let mut terminal = ratatui::try_init()?;

    let res = app.run(&mut terminal);

    ratatui::try_restore()?;

    // if any error has occured while executing, print it in cooked mode
    res.inspect_err(|e| println!("error: {e:?}"))?;

    Ok(())
}
