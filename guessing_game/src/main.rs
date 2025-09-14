use std::io;
use rand::Rng;

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    let mut guess: String = String::new();

    println!("Guess the number!");
    println!("Please input your guess.");


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    println!("The secret number is: {secret_number}");

}
