use std::collections::HashMap;

pub fn make_hand() -> HashMap<i8, i8> {
    let mut hand = HashMap::new();
    let hand_size = 4;
   
    let mut a = super::card::Card::make_card(5, true);

    hand.insert(8_i8, 1_i8);
    
//    read::read_i8();

    for (a, b) in &hand {
        println!("{}: {}", a, b);
    };
    hand
}

