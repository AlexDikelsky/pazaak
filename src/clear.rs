use std::process::Command;

pub fn clear() {
    let mut c = Command::new("clear");
    c.status().expect("Failed");
}

