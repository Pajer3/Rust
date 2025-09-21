fn main() {
    let mut gifts: Vec<&'static str> = Vec::new();

    for current_day in 1..=12 {
        println!("
            On the {} of christmas my true love gave me:", 
            day_name(current_day)
        );

        gifts.push(get_gift(current_day));

        for gift in gifts.iter().rev() {
            println!("{gift}")
        }

        println!("");
    }
}

fn day_name(x: u8) -> &'static str {
    match x {
        1 => "First",
        2 => "Second",
        3 => "Third",
        4 => "Fourth",
        5 => "Fifth",
        6 => "Sixth",
        7 => "Seventh",
        8 => "Eighth",
        9 => "Ninth",
        10 => "Tenth",
        11 => "Eleventh",
        12 => "Twelfth",
        _ => "Invalid day",
    }
}

fn get_gift(x: u8) -> &'static str {
    match x {
        1 => "A partridge in a pear tree.",
        2 => "Two turtle doves,",
        3 => "Three French hens,",
        4 => "Four calling birds,",
        5 => "Five golden rings,",
        6 => "Six geese a-laying,",
        7 => "Seven swans a-swimming,",
        8 => "Eight maids a-milking,",
        9 => "Nine ladies dancing,",
        10 => "Ten lords a-leaping,",
        11 => "Eleven pipers piping,",
        12 => "Twelve drummers drumming,",
        _ => "Invalid day",
    }
}
