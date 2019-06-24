//use colored::*;
use rand::prelude::*;

#[derive(Debug)]
pub struct Card {
    value: i8,    
}

impl Card {
    pub fn make_card(value_u: i8) -> Card {
       Card { value: value_u }
    } 
    pub fn empty_card() -> Card {
        Card { value: 0 }
    }
    pub fn make_rand_card() -> Card {
        Card { value: rand_not_zero() }
    }
    pub fn make_draw_card() -> Card {
        let mut rng = rand::thread_rng();
        Card { value: rng.gen_range(1, 10) }
    }
    pub fn get_value(&self) -> i8 {
        self.value
    }
    pub fn mark_used(&mut self) -> () {
        self.value = 0     
    }
    pub fn mid_line(&self) -> String {
        let x = if &self.value == &0 {
            " ".to_string()
        }
        else {
            self.value.to_string()
        };
        ("| ".to_string() + &sign(self.value).to_string() + &x +  &" |".to_string())
    }
}
fn rand_not_zero() -> i8 {
    let mut rng = rand::thread_rng();
    if rand::random() {
        rng.gen_range(-6, 0)
    }
    else {
        rng.gen_range(1, 7)
    }
}
fn sign(x: i8) -> String {
    if x == 0 {
        " ".to_string()
    }
    else if x == x.abs() {
        "+".to_string()
    }
    else {
        "".to_string()
    }
}
