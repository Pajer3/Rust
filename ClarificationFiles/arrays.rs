use std::io;

fn main() {
    let a: [i32; 5] = [0,1,2,3,4];
    let mut index = String::new();


    println!("Write a number");
    match io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line") {
        0 => println!("No input"),
        _ => continue,
    }

    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number!");

    let element = a[index];


    println!("Element at index {} is {}", index, element);

}