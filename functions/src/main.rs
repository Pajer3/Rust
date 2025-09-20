use std::io;

fn main() {
    println!("Write a number");

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line");

    let x: i32 = answer
        .trim()
        .parse()
        .expect("Please type a number");

    another_function(x);
}

fn another_function(number: i32)  {
    println!("the number is {number}");
}
