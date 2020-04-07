use std::io::stdin;

fn main() {
    println!("Welcome to Guess the Number game!");

    println!("Please enter your guess: ");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect(
        "Failed to read the line",
    );

    println!("You guessed: {}", guess);
}
