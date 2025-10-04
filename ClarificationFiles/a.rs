fn main() {
    let s1 = String::from("Hello, ");
    for (i, s) in <Char as func>::my_chars(&s1) {
        println!("{i} an {s}")
    }
    // println!("{:?}", s1.chars().next())
}

trait func {
   fn my_chars(s: &str) -> Vec<(usize, char)>;
}

pub struct Char;

impl func for Char {
    fn my_chars ( s: &str ) -> Vec < ( usize, char ) > {
        let mut out: Vec::<(usize, char)> = Vec::new();
        let mut i: usize = 0;

        for c in s.chars() {
            out.push((i, c));
            i += c.len_utf8();
        }

        out
    }
}
