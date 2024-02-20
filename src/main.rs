use wasm_game_of_life::Universe;

fn main() {
    // 64×64 max: 3500
    // 32×32 max: 80
    // 38×38 max: 190
    let n = 190;

    let mut universe = Universe::new(38, 38);

    for _ in 0..n {
        universe.tick();
        // clearing
        print!("{}[2J", 27 as char);
        // std::process::Command::new("clear");

        println!("{}", universe);

        // sleeping
        std::thread::sleep(std::time::Duration::from_millis(24));
    }
}
