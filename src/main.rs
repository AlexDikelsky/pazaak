mod hand;
mod card;
mod read;
mod player;
mod turn;
mod game;
mod clear;
mod disp_board;
mod series;

pub use crate::card::*;
pub use crate::hand::*;
pub use crate::read::*;
pub use crate::player::*;
pub use crate::turn::*;
pub use crate::game::*;
pub use crate::clear::*;
pub use crate::disp_board::*;
pub use crate::series::*;

static HAND_SIZE: u8 = 4;
static POINTS_TO_WIN: i8 = 20; //Needs to be signed because compared to points, which can be a negative number

fn main() {
    let mut p1 = player::Player::make_ai("Player 1", 0);
    let mut p2 = player::Player::make_ai("Player 2", 1);

    disp_board::disp(&p1, &p2);
    series::play_series(&mut p1, &mut p2);
}
