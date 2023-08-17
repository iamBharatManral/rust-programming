#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;
fn main() {
    let mut user1 = User{
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1
    };
    println!("User1: {:?}", user1);
    user1.sign_in_count = 2;
    println!("Sign-in count: {:?}", user1.sign_in_count);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("User2: {:?}", user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Color: {:?}, Origin: {:?}", black, origin);

    let subject = AlwaysEqual;
    println!("Unit Struct: {:?}", subject);

}
