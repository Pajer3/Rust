use rand::Rng;
use std::{cmp::Ordering,io};

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess: String = String::new();
        println!("Guess the number!");
    
        if io::stdin().read_line(&mut guess).is_err() {
            eprintln!("Failed to read line. Please try again.");
            continue;
        }
    
        let guess: u32 = match guess
            .trim()
            .parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
    
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
