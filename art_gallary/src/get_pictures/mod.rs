pub mod large {
    pub fn simulate() {
        println!("Large_picture");
    }
}

pub mod small {

    pub use super::large::simulate as sim;

    pub fn run() {
        sim();
    }
}