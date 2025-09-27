#[derive(Debug)]
struct Circle {
    radius: u32
}

impl Circle {
    fn area(&self) -> f64 {
        (self.radius * 2) as f64
    }

    fn can_hold(&self, other: &Circle) -> bool {
        self.radius > other.radius
    }
}

fn main() {
    let circel1: Circle = Circle {
        radius: 50
    };

    let circel2: Circle = Circle {
        radius: 20
    };

    if circel1.can_hold(&circel2) {
        println!("Can hold")
    } else {
        println!("Can't hold")
    }
}