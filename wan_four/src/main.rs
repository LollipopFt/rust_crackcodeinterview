use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn palcheck(strg: String) -> Vec<char> {
    // let check: bool;
    let map: HashMap<char, u8> = HashMap::new();
    let slist: Vec<char> = strg.trim().chars().collect();
    for i in slist {
        if map.contains_key(&i) {
            let stat = map.entry(i).or_insert(0);
            *stat += 1;
        } else {
            map.entry(i).or_insert(1);
        }
    };
    if slist.len()%2!=0 {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        assert_eq!(palcheck("lollipop ft  ".to_string()), vec!['l', 'o', 'l', 'l', 'i', 'p', 'o', 'p', ' ', 'f', 't']);
    }
}