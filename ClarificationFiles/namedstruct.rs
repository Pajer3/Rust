#[derive(Debug)]
struct Pair {
    x: i32,
    y: i32
}

impl Pair {
    fn distance_from_origin(&self) -> () {
       let sum: f64 = (self.x.pow(2) + self.y.pow(2)) as f64;
        println!("{:#?}",sum )
    }
}

fn main() {
    let origin: Pair = Pair {
        x: 53,
        y: 23
    };
    
    origin.distance_from_origin();
}