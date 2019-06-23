//use ansi_term::style
//use rng::Rand;

pub struct Card {
    value: i8,
    used: bool,
    //flip: bool,
}

impl Card {
    pub fn make_card(value_u: i8) -> Card {
        Card { value: value_u, used: false, /*flip: false*/ }
    }
    /*pub fn make_flip_card(value_u: i8, flip_u: bool) -> Card {
        Card { value: value_u, used: false, /*flip: flip_u*/ }
    }*/
    pub fn empty_card() -> Card {
        Card { value: 0, used: true, /*flip: false*/ }
    }
    /*pub fn make_rand_card() -> Card {
        let mut rng = rand::thread_rng();
        Card { value: rng.gen_range(1, 11), used: false, /*flip: false*/ }
    }*/
    pub fn get_value(&self) -> i8 {
        self.value
    }
    /*pub fn flip_value(&mut self) -> () {
        if self.flip {
            self.value = self.value * -1_i8;
        }
        else {
            println!("Cannot be flipped");
        }
    }*/
    pub fn mark_used(&mut self) -> () {
        if !self.used {
            self.used = true;
        }
        else {
            println!("Error, used card twice or more");
        }
    }
}

