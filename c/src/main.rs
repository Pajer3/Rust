use c::clamp_nonnegative;

fn main() {
    let mut array: [i32; 6] = [4,-5,-3,2,3,4];
    println!("{:#?}", clamp_nonnegative(array));
}