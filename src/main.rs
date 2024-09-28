use app::App;

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

    let mut terminal = ratatui::try_init()?;

    let mut app = App::default();
    let res = app.run(&mut terminal);

    ratatui::try_restore()?;

    // if any error has occured while executing, print it in cooked mode
    res.inspect_err(|e| println!("error: {e:?}"))?;

    Ok(())
}
