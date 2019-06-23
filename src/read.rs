use std::io;
use crate::HAND_SIZE;

pub fn read_i8() -> i8 {
    string_to_i8(read_string()) 
}
pub fn read_bounded_u8() -> u8 {
    let mut x = 0;
    let mut valid_input = false;
    while !valid_input {
        x = string_to_u8(read_string());
        if x > HAND_SIZE {
            println!("Try again");
        }
        else {
            valid_input = true;
        }
    }
    x
}
//Asks user for a string and returns it
fn read_string() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("Failed to read line");
    x    
}

//Takes a string and returns a u32. If can't be parsed, returns 0
fn string_to_i8(x: String) -> i8 {
    let x: i8 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    x
}
fn string_to_u8(x: String) -> u8 {
    let x: u8 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    x
}
