//use ansi_term::style
use rand::prelude::*;

pub struct Card {
    value: i8,    
}

impl Card {
    pub fn make_card(value_u: i8) -> Card {
        Card { value: value_u, }
    } 
    pub fn empty_card() -> Card {
        Card { value: 0 }
    }
    pub fn make_rand_card() -> Card {
        let mut rng = rand::thread_rng();
        Card { value: rng.gen_range(-6, 7) }
    }
    pub fn make_draw_card() -> Card {
        let mut rng = rand::thread_rng();
        Card { value: rng.gen_range(1, 11) }
    }
    pub fn get_value(&self) -> i8 {
        self.value
    }
    pub fn mark_used(&mut self) -> () {
        self.value = 0     
    }
}

