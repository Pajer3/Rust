pub fn sum_to(mut n: u32) -> u32 {
    let mut total: u32 = 0;
    
    if n == 0 {
       return 0
    }

    for i in 1..=n {
       println!("{}",i);
       total = i + total; 
    }

    total
}


pub fn sum_if_even(n: u32) -> u32 {
    if n % 2 != 0 {
        return println!("Not a even number");
    } else {
        let mut total = 0;
        for i in 0..=n {
            total += i;
        }
        total
    }
}

pub fn is_even(number: i32) -> bool {
    number % 2 == 0
}

pub fn match_sum(number: Option<i32>) -> i32 {
    if let Some(n) = number {
        let mut total = 0;
        for number in 0..n {
            total += number;
        } 
        return total;
    }
}


pub fn match_sum_if_even(number: i32) -> i32 {
    if number % 2 != 0 {
        402
    } else {
        let mut total = 0;
        for n in 0..number {
            total += n;
        }
        total
    }
}

pub fn sum_of_squares(nums: Vec<i32>) {
    let mut total: i32 = 0;
    
    for i in nums {
       total += i * i ;
    }

    return total
}

pub fn count_vowels(s: &str) -> i32 {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut count: i32 = 0;
    for l in s.chars() {
        for v in VOWELS.iter() {
            if l == v {
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn plus_one(v: &[i32]) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();

    for num in v {
        list.push(num + 1);
    }

    list
}

// Clamp negatives to zero (in-place)
// Goal: Mutate a slice so all negatives become 0; others stay the same.
// Signature: fn clamp_nonnegative(xs: &mut [i32])
// Rules: Use for x in xs.iter_mut(). No extra allocations.
// Example: [3, -2, 7, -1] → [3, 0, 7, 0].

pub fn clamp_nonnegative(xs: &mut [i32]) {
    for num in xs.iter_mut() {
        if *num < 0 {
            *num = 0;
        }
    }
}


// Reverse without helpers
// Goal: Return a reversed copy of the input, built manually.
// Signature: fn reversed(xs: &[i32]) -> Vec<i32>
// Rules: Use a for over index range with .rev() (e.g., 0..xs.len()). Do not call .reverse(), .iter().rev(), or collect.
// Example: [1, 2, 3] → [3, 2, 1], [] → [].

fn reversed(xs: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    for num in (0..xs.len()).rev() {
        result.push(xs[num]);
    }
    result
}


// Find max and its index
// Goal: Return the index and value of the first maximum element, or None if empty.
// Signature: fn argmax(xs: &[i32]) -> Option<(usize, i32)>
// Rules: Use for (i, &x) in xs.iter().enumerate(). Track best (idx, val). On ties, keep the earliest index.
// Example: [3, 9, 9, 1] → Some( (1, 9) ), [] → None.

fn argmax(xs: &[i32]) -> Option<(usize, i32)> {
    let mut result: Option<(usize, i32)> = None;
    for (i , &x) in xs.iter().enumerate() {
        let if Some(index) = result 

    }


}