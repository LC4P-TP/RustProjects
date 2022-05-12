use std::collections::HashMap;
use std::io::*;

pub fn count_symbol() {
    println!("Enter text");
    let mut text = String::new();
    stdin().read_line(&mut text).unwrap();
    text = text.trim().parse().unwrap();

    println!("Enter symbol you want find");
    let symbol_need_find: char = imput_symbol();
    
    println!("{:?}", find_symbol(text, symbol_need_find));
}

fn imput_symbol() -> char {
    loop {
        let mut imput_symbol = String::new();
        stdin().read_line(&mut imput_symbol).unwrap();

        let check_symbol: char = match imput_symbol.trim().parse(){
            Ok(char) => char,
            Err(_) => {
                println!("Please enter one symbol");
                continue;
            }
        };
        return check_symbol;
    }
}

fn create_dictionary(text: String) -> HashMap<char, i32> {
    let mut dictionary = HashMap::new();

    for symbol in text.chars() {
        let count = dictionary.entry(symbol).or_insert(0);
        *count += 1;
    }
    return dictionary;
}

fn find_symbol(text: String, symbol_need_find: char) -> (char, i32) {

    let dictionary = create_dictionary(text);

    if dictionary.contains_key(&symbol_need_find) {
        return (symbol_need_find, dictionary[&symbol_need_find]);
    }else {
        return (symbol_need_find, 0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_symbol_exist() {
        let test_text = String::from("hello");
        let test_symbol = 'l';
        
        let result = find_symbol(test_text, test_symbol);

        let expection = ('l', 2);

        assert_eq!(expection,  result);
    }

    #[test]
    fn test_find_symbol_not_exist() {
        let test_text = String::from("hello");
        let test_symbol = 'b';
        
        let result = find_symbol(test_text, test_symbol);

        let expection = ('b', 0);

        assert_eq!(expection,  result);
    }
}