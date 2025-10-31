use std::collections::HashMap;

enum MedianMode {
    Median(i32),
    Mode(i32),
    Errno(String),
}

fn main() {
    let numbers: Vec<i32> = vec![5,3,2,1,2,6,7]
}


fn median(list: Vec<i32>) -> MedianMode {
    let mut sorted_list: Vec<i32> = Vec::new();
    let length: usize = list.len();
    let mode: MedianMode = MedianMode::Mode;
    let mut map: HashMap<i32, i32> = HashMap::new();

    if length > 10 {
        let error_message: String = String::from("The list is to large for this function, due to peformance reasons. Use another method");
        return MedianMode::Errno(error_message);
    }

    for (i, num) in list.iter().enumerate() {
        if num < list[i] {
            sorted_list.push(list[num]);
        }
    }

    let median: MedianMode::Median = {
        if sorted_list == 
    };

    


    
    
    
    println!("The sorted list: {:#?}", sorted_list);
    return MedianMode::Median(median);
    return MedianMode::Mode(mode);

}

const fn is_even(n: Option<i32>) {

}