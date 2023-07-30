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

pub fn get_game_number(game_number: &mut String) -> u32{

    loop {
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
    }
}

