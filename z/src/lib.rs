use rand::Rng;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub trait Show {
    fn show(&self) -> String;
}

impl Show for Point {
    fn show(&self) -> String {
        format!("{},{}", self.x, self.y)
    }
}

pub fn print_show<T: Show>(item: &T) {
    println!("{}", item.show());
}


pub fn special_mulitply(x: i32,y: i32) -> i32 {
    let mut rng: i32 = rand::random::<i32>();
    let total = x * y;

    let result = total + rng;
    result
}
