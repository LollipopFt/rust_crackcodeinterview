use std::{
    io
};

fn main() {
    let mut string: String = String::new();
    io::stdin().read_line(&mut string).expect("Error.");
    let string = string.trim();
    let list = string.chars();
    let mut repeat: Vec<char> = vec![];
    let mut repeat_bool: bool = false;
    for letter in list {
        if repeat.contains(&letter) {
            repeat_bool = true;
            break;
        } else {
            repeat.push(letter);
            continue;
        }
    }

    if repeat_bool {
        println!("Repeat value found.");
    } else {
        println!("No repeat value found.");
    }
}