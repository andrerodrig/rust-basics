use std::io;

pub mod guessing_game;
pub mod garden;
pub mod utils;

fn main() {
    println!("-----------------------SATAN GAMES-----------------------:");
    println!("Choose the game:");
    println!("1: Guessing Game");
    println!("2: Garden");

    let mut game_number = String::new();

    io::stdin()
        .read_line(&mut game_number)
        .expect("Failed to read the game number!");

    let game_number = utils::get_game_number(&mut game_number);

    if game_number == 1 {
        guessing_game::guessing_game();
    }
    else if game_number == 2 {
        garden::run_garden()
    }
}


