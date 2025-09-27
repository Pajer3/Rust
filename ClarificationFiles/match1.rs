fn describe_number(x: Option<i32>) {
    match x {
        Some(..-1) => println!("negative number"),
        None => println!("No number given"),
        Some(n) => println!("The number is {}", n),
    }
}

fn main() {
    let a = Some(42);
    let b: Option<i32> = None;
    let c = -50;

    describe_number(a);
    describe_number(b);
    describe_number(Some(c));
}
