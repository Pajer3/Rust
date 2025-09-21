use std::io;

fn main() {
    println!("Welcome to the temprature converter");
    println!("Would you like to \n
    A: Convert to Celius \n
    B: Convert to Fahrenheit \n

    Press number: 1 for A, and number 2 for B.
    ");

    'outer_loop: loop {
        let mut user_number: String = String::new();

        if io::stdin().read_line(&mut user_number).is_err() {
            eprintln!("Failed to capture a valid number");
            continue;
        } 

        let user_number: f64 = match user_number.trim().parse::<f64>()
        {
            Ok(user_number) = f64,
            Err(_) => continue,
        };
        
        if user_number == 1 {
            convert_to_celius(user_number);
        } else  if  user_number == 2 {
            convert_to_fahrenheit(user_number.to_string().trim().parse::<f64>());
        } else {
            break 'outer_loop;
        }
    }


}

fn convert_to_celius(x: f64) -> String {
    let celsius = 5.0 / 9.0 * (x - 32);
    let result = println!("{celsius}");
    result
}

fn convert_to_fahrenheit(x: f64) -> String {
    let fahrenheit = x * 9.0 / 5.0 + 32.0;
    let result = println!("{fahrenheit}");
    result
}