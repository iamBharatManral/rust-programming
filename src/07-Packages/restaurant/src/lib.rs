use std::fmt::Result;
use std::io::Result as IOResult;
use std::io::{self, Write};

mod front_of_house;
mod back_of_house;
use front_of_house::hosting;

fn deliver_order(){}

pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries")

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    hosting::add_to_waitlist();
}
