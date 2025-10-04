fn main() {
    let a: String = "Hello".to_string();
    let b: String = "café".to_string();
    let c: String = "naïve".to_string();
    let d: String = "Здравствуйте".to_string();
    let e: String = "😊".to_string();
    let f: String = "🇳🇱".to_string();

    let words = vec![a, b, c, d, e, f];

    for (bytes, ch) in words[3].char_indices() {
        println!("{}: {}", bytes, ch);
    }

    let owned_vec = data_summary(&words, None);

    println!("{:#?}", owned_vec);
}

pub(crate) fn transform_to_vec(x: &[String]) -> Vec<String> {
    let mut empty_vec: Vec<String> = Vec::with_capacity(x.len());


    for s in x {
        empty_vec.push(s.to_string());
    }

    empty_vec
}


fn data_summary(
    string_data: &[String],
    numbers_data: Option<&[i32]>
) -> Vec<(String, char, u8)> {
    let mut result: Vec<(String, char, u8)> = Vec::new();

    if let Some(_nums) = numbers_data {
        println!("Nice, you got some numbers data but I don’t have a sum for that yet.");
    } else {
        result = forms(string_data);
    }

    fn forms(a: &[String]) -> Vec<(String, char, u8)> {
        let mut x: Vec<(String, char, u8)> = Vec::new();

        for item in a {
            if let Some(ch) = item.chars().next() {
                if ch.is_ascii() {
                    let ascii_value = ch as u8;
                    x.push((item.clone(), ch, ascii_value));
                }
            }
        }

        x
    }

    result
}
