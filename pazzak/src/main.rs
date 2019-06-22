mod hand;
mod card;
pub use crate::card::*;
pub use crate::hand::*;

fn main() {
    println!("Hello, world!");
    let x = card::Card::make_card(5, true);
    println!("{}", x.get_value());
}
