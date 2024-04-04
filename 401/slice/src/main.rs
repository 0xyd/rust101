fn main() {
    let mut s = String::from("hello world");
    let world = first_word(&s);
    s.clear(); // string is cleared out so the s is ""
    println!("{:?}", s);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}