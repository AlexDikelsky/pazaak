use std::collections::HashMap;
//TODO Add a random hand maker
pub fn make_hand() -> HashMap<u8, super::card::Card> {
    let mut hand = HashMap::new();
    let hand_size = 4;
   
    for i in 0..hand_size {
        hand.insert(i, super::card::Card::make_card(
                                super::read::read_i8()        
                    ));
    };

    hand
}
