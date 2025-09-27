struct Rectangle(u32, u32);

impl Rectangle {
    fn is_square(&self) -> bool {
        self.0 == self.1
    }
}

fn main() {
    let x: Rectangle = Rectangle(10,10);

    println!("{}",x.is_square());
}