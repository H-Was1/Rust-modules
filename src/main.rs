// #[derive(Debug)]
use rand::Rng;
use std::{fmt::Result, io};

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn gen_random_num() -> io::Result<String> {
    let random_num = rand::thread_rng().gen_range(1..=10);
    Ok(random_num.to_string())
}

fn play_game() -> io::Result<()> {
    println!("I'm thinking of a number between 1 and 10.");
    let random_num = gen_random_num().expect("Failed to generate random number");
    let mut tries = 0;
    loop {
        let st = format!("{}th", tries);
        let guess = get_user_input().expect("Failed to get user input");
        if guess == random_num {
            println!(
                "Congratulations! You guessed the correct number in the {} try. The correct answer is: {}",
                if tries == 1 {
                    "1st"
                } else if tries == 2 {
                    "2nd"
                } else if tries == 2 {
                    "3rd"
                } else {
                    &st
                },
                random_num
            );
            break;
        }
        if tries < 10 {
            println!("Sorry, that's not the number. {} tries left", 10 - tries);
        } else {
            println!("Sorry, You failed to get the number 10 times. Game over.");
            break;
        }
        tries += 1;
    }
    Ok(())
}

fn main() {
    println!("Welcome to the Number Guessing Game!");
    if let Err(e) = play_game() {
        println!("Error: {}", e);
    }
}
