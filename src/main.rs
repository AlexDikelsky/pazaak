mod hand;
mod card;
mod read;
mod player;
mod turn;
mod game;
mod clear;
mod disp_board;

pub use crate::card::*;
pub use crate::hand::*;
pub use crate::read::*;
pub use crate::player::*;
pub use crate::turn::*;
pub use crate::game::*;
pub use crate::clear::*;
pub use crate::disp_board::*;

static HAND_SIZE: u8 = 4;
static POINTS_TO_WIN: i8 = 20; //Needs to be signed because compared to points, which can be a negative number

fn main() {
    //clear::clear();
    let mut p1 = player::Player::make_ai();
    let mut p2 = player::Player::make_ai();

    disp_board::disp(&p1, &p2);

    game::setup_game(&mut p1, &mut p2);
    game::play_turns(&mut p1, &mut p2);

    /*let mut p1 = player::Player::make_ai();
    //println!("{}", x.get_score());
    
    for i in 0..4 {
        println!("{}: {}", i+1, p1.get_card(i).get_value());
    }

    turn::draw(&mut p1); 

    for i in p1.get_board().iter() {
        for j in i.iter() {
            print!("{}", j.get_value());
        }
        println!();
    }
        

    println!("{}", p1.get_score());*/

    //println!("{}", x.get_side());

    /* 
    let y = hand::make_rand_hand();
    for i in 0..4 {
        println!("{}: {}", i, y[&i].get_value());
    }*/
}
