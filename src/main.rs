fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let guess = get_user_input();

    println!("You guessed: {}", guess);
}

fn get_user_input() -> String {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input.trim().to_string()
}