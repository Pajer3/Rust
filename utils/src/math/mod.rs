pub fn avarage(list: Vec<i32>) -> i32 {
    let length: i32 = list.len() as i32;
    let mut total: i32 = 0;

    for number in list {
        total += number
    }

    let results = total / length;
    results

}