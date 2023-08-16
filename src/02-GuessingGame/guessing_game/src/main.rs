use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!(":: Guess the number #️⃣!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess 🔢");
        let mut guess = String::new();

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line! 😞");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You Won! 🎉");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        };
    }
}
