fn main() {

    let mut x: usize = 0;
    
    loop {
        x += 1;
        println!("{}",x);

        if x == 30 {
            break x * 2;
        }
    };
}
