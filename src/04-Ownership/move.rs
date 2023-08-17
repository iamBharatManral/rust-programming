fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{s}");

    let y = s.clone();
    println!("y: {y}");
    println!("s: {s}");

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    return_multiple()
}

fn takes_ownership(s: String) {
    println!("s: {s}");
}

fn makes_copy(x: i32) {
    println!("x: {x}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn return_multiple() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{s2}' is {len}.");
}
