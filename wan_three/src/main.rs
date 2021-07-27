use std::io::stdin;

fn main() {
    let mut stri: String = String::new();
    stdin().read_line(&mut stri).expect("Error.");
    let finale = urlify(stri);
    println!("{}", finale);
}

fn urlify(stri: String) -> String {
    let mut str_list: Vec<char> = vec![];
    let stri = stri.trim().chars();
    for c in stri {
        if c == ' ' {
            str_list.push('%');
            str_list.push('2');
            str_list.push('0');
        } else {
            str_list.push(c);
        }
    }
    let mut string: String = String::new();
    for i in str_list {
        string+=&i.to_string();
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failure() {
        assert_eq!(urlify("death note".to_string()), "deathnote".to_string());
    }

    #[test]
    fn success() {
        assert_eq!(urlify("hello world".to_string()), "hello%20world".to_string());
    }
}