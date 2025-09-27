#[derive(Debug)]
struct Pair(i32,i32);

impl Pair {
    fn swap(&mut self) {
        let temp = self.0;
        self.0 = self.1;
        self.1 = temp;
    }
}

fn main() {
    const FAV_NUMBERS: Pair = Pair(6,7);
    let mut numbers: Pair = FAV_NUMBERS;

    numbers.swap();

    dbg!(numbers);
}