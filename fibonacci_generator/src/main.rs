/*
    The Fibonacci sequence is a special number pattern. It goes like this:

    0, 1, 1, 2, 3, 5, 8, 13, 21, ...


    Here’s how it works:

    It starts with two fixed numbers: 0 and 1.

    Every number after that is created by adding the previous two numbers.

    0 + 1 = 1

    1 + 1 = 2

    1 + 2 = 3

    2 + 3 = 5

    3 + 5 = 8

    and so on forever.

    So the sequence is built step by step.
*/


fn main() {
    let mut fib_one: u128 = 0;
    let mut fib_two: u128 = 1;
    
    loop { 
        let result: u128 = fib_one + fib_two;

        println!("{} + {} = {}", fib_one, fib_two, result);

        fib_one = fib_two;
        fib_two = result;
    }
}
