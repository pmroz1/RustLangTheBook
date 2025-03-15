use rand::rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = get_random_number();

    loop {
        let guess: u32 = get_user_input();

        // println!("The secret number is: {}", random_number);
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_random_number() -> u32 {
    use rand::Rng;
    let secret_number = rng().random_range(1..=100);
    secret_number
}

fn get_user_input() -> u32 {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().parse().expect("Please type a number!")
}
