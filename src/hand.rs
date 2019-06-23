static HAND_SIZE: u8 = 4;

pub fn make_hand() -> Vec<super::card::Card> {
    let mut hand = vec![];
    for _i in 0..HAND_SIZE {
        hand.push(super::card::Card::make_card(
                        super::read::read_i8()        
                    ));
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

