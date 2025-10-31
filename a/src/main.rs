fn main() {
    let vault: Vec<i64> = vec!([1,0,1,0,1,0]);

}


fn sort(list: Inda) -> {
    match list {
        Inda::Text(_) | Inda::Sent(_) | Inda::Vec_List(_) {
            println!("Works only on a list of numbers")
            None
        }
        Inda::Array_List(nums) => Some(manual_sort(nums.to_vec()));
        Inda::Number(nums) => Some(manual_sort(nums));
    }
}

#[derive(Debug)]
enum Inda {
    Text(Vec<String>),
    Numbers(Vec<i64>),
    Array_List(&'static [i64]),
    Vec_List(Vec<Vec<i64>>),
    Sent(String),
}


fn manual_sort(mut list: Vec<i64) -> {
    let mem: Vec<i64> = Vec::new();

    for nums in list {
        if mem.is_empty(){
            mem.push(nums);
            continue;
        }

        
    }

}