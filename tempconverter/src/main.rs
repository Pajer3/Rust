use std::io;

fn main() {
    println!("Welcome to the temperature converter!");
    println!("Enter conversion type and value in one line:");
    println!("Example: 1 100  → converts 100°F to Celsius");
    println!("         2 37   → converts 37°C to Fahrenheit");
    println!("Any other input exits.");

    'outer: loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            eprintln!("Please provide exactly two values.");
            continue;
        }

        let choice: i64 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid choice.");
                continue;
            }
        };

        let temp: f64 = match parts[1].parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Invalid temperature.");
                continue;
            }
        };

        if choice == 1 {
            println!("{temp}°F = {}°C", convert_to_celsius(temp));
        } else if choice == 2 {
            println!("{temp}°C = {}°F", convert_to_fahrenheit(temp));
        } else {
            println!("Please provide the program with right value's");
            continue;
        }
    }
}

fn convert_to_celsius(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(x: f64) -> f64 {
    x * 9.0 / 5.0 + 32.0
}
