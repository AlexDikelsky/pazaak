mod hand;
mod card;
mod read;
mod player;

pub use crate::card::*;
pub use crate::hand::*;
pub use crate::read::*;
pub use crate::player::*;

fn main() {
    let x = player::Player::make_player();
    println!("{}", x.get_score());
    
    for i in 0..4 {
        println!("{}: {}", i, x.get_card(i).get_value());
    }
}
