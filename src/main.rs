// #[derive(Debug)]
use rand::Rng;
use std::{fmt::Result, io};

fn get_user_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

enum operations {
    shut_down,
    hibernate,
    restart,
    sleep,
    power_off,
}

fn main() {
    println!("Choose a command.");
    let user_input = get_user_input().expect("Failed to get user input.");
    let operation: operations = match user_input.as_str() {
        "shutdown" => operations::shut_down,
        "hibernate" => operations::hibernate,
        "restart" => operations::restart,
        "sleep" => operations::sleep,
        "poweroff" => operations::power_off,
        _ => {
            println!("Invalid command. Please try again.");
            return;
        }
    };
    match operation {
        operations::shut_down => println!("Shutting down..."),
        operations::hibernate => println!("Hibernating..."),
        operations::restart => println!("Restarting..."),
        operations::sleep => {
            let seconds = rand::thread_rng().gen_range(1..=10);
            println!("Sleeping in {} seconds...", seconds);
            std::thread::sleep(std::time::Duration::from_secs(seconds));
        }
        operations::power_off => println!("Powering off..."),
    }
}
