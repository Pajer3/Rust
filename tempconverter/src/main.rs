use std::io;

fn main() {
    println!("Welcome to the temperature converter!");

    'outer: loop {
        println!("Choose an option:");
        println!("1: Convert Fahrenheit → Celsius");
        println!("2: Convert Celsius → Fahrenheit");
        println!("Any other key to quit");

        // First input: menu choice
        let mut choice = String::new();
        if io::stdin().read_line(&mut choice).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let choice: i64 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid choice.");
                continue;
            }
        };

        // Exit if not 1 or 2
        if choice != 1 && choice != 2 {
            println!("Please write a number 1 or 2");
            continue;
        }

        // Second input: temperature value
        println!("Enter the temperature value:");
        let mut temp_input = String::new();
        if io::stdin().read_line(&mut temp_input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let temp: f64 = match temp_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid number.");
                continue;
            }
        };

        // Perform conversion
        if choice == 1 {
            println!("{temp}°F = {}°C", convert_to_celsius(temp));
        } else {
            println!("{temp}°C = {}°F", convert_to_fahrenheit(temp));
        }
    }
}

fn convert_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(x: f64) -> f64 {
    x * 9.0 / 5.0 + 32.0
}
