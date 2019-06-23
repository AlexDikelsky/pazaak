mod hand;
mod card;
mod read;
mod player;
mod turns;

pub use crate::card::*;
pub use crate::hand::*;
pub use crate::read::*;
pub use crate::player::*;
pub use crate::turns::*;

fn main() {
    let mut x = player::Player::make_ai();
    //println!("{}", x.get_score());
    
    for i in 0..4 {
        println!("{}: {}", i, x.get_card(i).get_value());
    }

    x.add_card(&card::Card::make_card(3));

    for i in x.get_board().iter() {
        for j in i.iter() {
            print!("{}", j.get_value());
        }
        println!();
    }
    
    

    println!("{}", x.get_score());

    //println!("{}", x.get_side());

    /* 
    let y = hand::make_rand_hand();
    for i in 0..4 {
        println!("{}: {}", i, y[&i].get_value());
    }*/
}
