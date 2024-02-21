use wasm_game_of_life::*;

fn main() {
    // if not randomly generated:
    //     64×64 max: 3500
    //     32×32 max: 80
    //     38×38 max: 190
    let n = 8000;

    let mut universe = Universe::from_figur(32, 32, featherweigth_spaceship());

    // println!("{}", universe);

    for _ in 0..n {
        universe.tick();
        // clearing
        print!("{}[2J", 27 as char);
        // std::process::Command::new("clear");

        println!("{}", universe);

        // sleeping
        std::thread::sleep(std::time::Duration::from_millis(80));
    }
}
