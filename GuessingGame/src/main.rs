use rand::rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = get_random_number();

    loop {
        match get_user_input() {
            Ok(guess) => {
                println!("You guessed: {}", guess);

                match guess.cmp(&secret_number) {
                    Ordering::Less => println!("Too small!"),
                    Ordering::Greater => println!("Too big!"),
                    Ordering::Equal => {
                        println!("You win!");
                        return;
                    }
                }
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
}

fn get_random_number() -> u32 {
    use rand::Rng;
    let secret_number = rng().random_range(1..=100);
    secret_number
}

fn get_user_input() -> Result<u32, &'static str> {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    match user_input.trim().parse::<u32>() {
        Ok(num) => Ok(num),
        Err(_) => {
            println!("Please enter a valid number");
            get_user_input()
        }
    }
}
