use std::{
    io::stdin,
    collections::HashMap
};

fn main() {
    let mut str1: String = String::new();
    stdin().read_line(&mut str1).expect("Error.");
    let mut str2: String = String::new();
    stdin().read_line(&mut str2).expect("Error.");
    let permutation = check(str1, str2);
    if permutation == true {
        println!("It is a permutation.");
    } else {
        println!("It is not a permutation.");
    }
}

fn check(str1: String, str2: String) -> bool {
    let str_map = convert(str1);
    let str2_map = convert(str2);
    let permutation: bool;
    if str_map == str2_map {
        permutation = true;
    } else {
        permutation = false;
    }
    permutation
}

fn convert(string: String) -> HashMap<char, u8> {
    let string = string.trim().chars();
    let mut str_vec: Vec<char> = vec![];
    let mut map: HashMap<char, u8> = HashMap::new();
    for ch in string {
        if ch == ' ' {
            continue;
        } else {
            str_vec.push(ch);
        }
    };
    for letter in str_vec {
        if map.contains_key(&letter) {
            let stat = map.entry(letter).or_insert(0);
            *stat += 1;
        } else {
            map.entry(letter).or_insert(1);
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trues() {
        let str1 = "hello world".to_string();
        let str2 = "whoerllldo".to_string();
        assert!(check(str1, str2));
    }

    #[test]
    fn falses() {
        let str1 = "tuhenbn".to_string();
        let str2 = "thotuho".to_string();
        assert!(!check(str1, str2));
    }
}