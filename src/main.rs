use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("-----------------------SATAN GAMES-----------------------:");
    println!("Choose the game:");
    println!("1: Guessing Game");
    println!("2: Pendule");

    let mut game_number = String::new();

    io::stdin()
        .read_line(&mut game_number)
        .expect("Failed to read the game number!");

    let game_number = get_game_number(&mut game_number);

    if game_number == 1 {
        guessing_game();
    }
}


fn convert_to_number(input_str: &mut String) -> u32{

    loop {
         match input_str.trim().parse() {
           Ok(input_str) => break input_str,
           Err(_) => {
               println!("Please type a number!");
               continue;
           },
        };
    }
}

fn get_game_number(game_number: &mut String) -> u32{

    return loop {
        let number = convert_to_number(game_number);
        if number == 1 {
            break number;
        }
        else if number == 2 {
            break number;
        }
        else {
            println!("Choose between the options above!");
            continue;
        };
    };
}

fn guessing_game() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            },
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
