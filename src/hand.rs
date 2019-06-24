use crate::HAND_SIZE;

pub fn make_hand() -> Vec<super::card::Card> {
    super::clear::clear();
    let mut hand = vec![];
    println!("Choose your hand!");
    println!("Pick {} numbers between -6 and 6 not including 0.", HAND_SIZE);
    for i in 0..HAND_SIZE {
        println!("Card {}: ", i+1);
        let x = super::read::read_non_zero_i8();
        hand.push(super::card::Card::make_card(x));
    };
    hand
}
pub fn make_rand_hand() -> Vec<super::card::Card> {
    let mut hand = vec![];
    for _i in 0..HAND_SIZE {
        hand.push(super::card::Card::make_rand_card());
    };
    hand
}

