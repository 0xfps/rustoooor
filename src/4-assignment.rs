// Assignments.

use std::io;

fn main() {
    // Tried to build in a way that allows a user to pick the game they want based on indexes.
    const GAMES: [&str; 3] = [
        "Celsius to Fahrenheit",
        "Christmas Carol",
        "Nth Fibonacci Number",
    ];

    println!(
        "
        -------------------------------------------
            WELCOME TO fps'S FIRST RUST GAME!!!
        -------------------------------------------

        -------------------------------------------
        OPTIONS (PRESS A NUMBER TO SELECT)
        -------------------------------------------

        -------------------------------------------
        1: CELSIUS TO FAHRENHEIT CONVERTER
        2: CHRISTMAS CAROL
        3: NTH fibonacci NUMBER
        -------------------------------------------

        -------------------------------------------
        AWAITING INPUT...
        -------------------------------------------
    "
    );

    for game in GAMES {
        println!("{}", game);
    }

    'select_game_loop: loop {
        let mut selected_game = get_user_input("CHOOSE GAME");

        if (selected_game.1 == true) {
            if (selected_game.0 == 1) {
                celsius_to_fahrenheit();
                break;
            } else if selected_game.0 == 2 {
                christmas_carol();
                break;
            } else if selected_game.0 == 3 {
                nth_fibonacci_number();
                break;
            } else {
                println!("Invalid game!");
            }
        } else {
            continue;
        }
    }
}

fn celsius_to_fahrenheit() {
    let celsius_number = get_user_input("INPUT A NUMBER IN CELSIUS").0 as f64;
    let nine_over_five = 9.00 / 5.00;
    let fahrenheit: f64 = (celsius_number * nine_over_five) + 32.00;
    println!(
        "The fahrenheit equivaent of {} degree Celsius is {} degrees Fahrenheit",
        celsius_number, fahrenheit
    )
}

fn christmas_carol() {
    let starting_string = "On the ";
    let middle_string: &str = " day of Christmas, my true love gave to me, ";
    let (days, gifts) = days_and_gifts();

    const LENGTH: usize = 12;
    let mut starting_point: usize = 0;

    while starting_point < LENGTH {
        if (starting_point == 0) {
            println!(
                "{} {} {} {}. \n",
                starting_string, days[starting_point], middle_string, gifts[starting_point]
            );
        } else {
            let mut new_str = starting_string.to_string()
                + days[starting_point].as_str()
                + middle_string
                + gifts[starting_point].as_str()
                + ", ";
            for i in (0..starting_point).rev() {
                if i == 0 {
                    new_str = new_str + "and " + gifts[i].as_str() + ".";
                } else {
                    new_str = new_str + gifts[i].as_str() + ", "
                }
            }

            println!("{} \n", new_str);
        }

        starting_point = starting_point + 1;
    }
}

fn nth_fibonacci_number() {
    let input = get_user_input("INPUT A NUMBER").0;
    if (input > 50) {
        println!("We can't go past 50.");
    }

    if (input == 0) || (input == 1) {
        println!("{}", 0);
    } else if (input == 2) || (input == 3) {
        println!("{}", 2);
    } else {
        let mut v = vec![1, 1];

        let number_of_iterations = input - 4;
        let mut count = 0;

        while count <= number_of_iterations {
            let mut next_num = v[v.len() - 1] + v[v.len() - 2];
            v.push(next_num);
            count += 1;
        }

        println!(
            "The fibonacci sequence at index {} is {}.",
            input,
            v[v.len() - 1]
        )
    }
}

fn get_user_input(prompt: &str) -> (u64, bool) {
    println!("{}", prompt);

    let mut input = 0;

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    match text.trim().parse() {
        Ok(num) => input = num,
        Err(_) => {
            return (0, false);
        }
    };

    (input, true)
}

fn days_and_gifts() -> ([String; 12], [String; 12]) {
    let DAYS: [String; 12] = [
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::from("fourth"),
        String::from("fifth"),
        String::from("sixth"),
        String::from("seventh"),
        String::from("eighth"),
        String::from("ninth"),
        String::from("tenth"),
        String::from("eleventh"),
        String::from("twelveth"),
    ];

    let GIFTS: [String; 12] = [
        String::from("a patridge in a pear tree"),
        String::from("two turle doves"),
        String::from("three French hens"),
        String::from("four calling birds"),
        String::from("five golden rings"),
        String::from("six geese a'laying"),
        String::from("seven swans a'swimming"),
        String::from("eight maids a'milking"),
        String::from("nine ladies dancing"),
        String::from("ten lords a'leaping"),
        String::from("eleven drummers drumming"),
        String::from("twelve pipers piping"),
    ];

    (DAYS, GIFTS)
}
