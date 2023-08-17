fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("first word in s: {word}");
    let s2 = "bye bye";
    let word = first_word(&s2);
    println!("first word in s2: {word}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
