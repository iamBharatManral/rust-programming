#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u32{
    match coin{
        Coin::Penny => 1,
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter => 25
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        Some(num) => Some(num + 1),
        None => None
    }
}


fn main() {
    absent_value();
    cents();
    plus_one_example();
    dice_roll();
    if_let();
}

fn absent_value(){
    let some_number = Some(5);
    let some_char = Some('c');

    let absent_number : Option<i32> = None;

    println!("\nSome No: {:?}, Char: {:?}, Absent: {:?}", some_number, some_char, absent_number);
}

fn cents(){
    let cents = value_in_cents(Coin::Quarter);
    println!("\nCents: {:?}", cents);
}

fn plus_one_example(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("\nSome(5) + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);
}

fn dice_roll(){
    let dice_roll = 9;
    println!("\nRolled: {dice_roll}");
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => re_roll()
    }
}

fn if_let(){
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("\nThe maximum is configured to be {max}");
    }
}
fn add_fancy_hat(){
    println!("adding fancy hat");
}

fn remove_fancy_hat(){
    println!("removing fancy hat");
}

fn move_player(other: i32){
    println!("moving player to {}", other);
}

fn re_roll(){
    println!("re-rolling")
}
