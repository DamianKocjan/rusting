use std::io;

use rand::thread_rng;
use rand::Rng;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn to_number(number: &String) -> u32 {
    number.trim().parse().expect("Not a number!")
}

fn main() {
    let mut rng = thread_rng();
    let number: u32 = rng.gen_range(0..100);
    let mut number_of_guesses: u32 = 0;

    loop {
        let guess = to_number(&get_input());

        if number == guess {
            break;
        } else if number > guess {
            println!("Too small!");
        } else {
            println!("Too high!");
        }

        number_of_guesses += 1;
    }

    println!("You won!");
    println!(
        "It took you {} times to guess the number",
        number_of_guesses
    );
}
