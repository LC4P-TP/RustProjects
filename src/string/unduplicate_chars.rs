use std::collections::*;

pub fn unduplicate_chars(line: String) -> HashSet<char> {
    let mut result = HashSet::new();

    for symbol in line.chars() {
        result.insert(symbol);
    }

    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unduplicate_chars_word() {
        let test_text = String::from("wwwooolf");

        let result = unduplicate_chars(test_text);
        let expection = HashSet::from(['w', 'l', 'o', 'f']);

        assert_eq!(expection, result);
    }

    #[test]
    fn test_unduplicate_chars_text() {
        let test_text = String::from("Wwooolf inn thhe    forest");

        let result = unduplicate_chars(test_text);
        let expection = HashSet::from([
            'o', 'n', 't', 'r', 'W', 'e', 'i', 's', 'h', 'f', 'w', 'l', ' ',
        ]);

        assert_eq!(expection, result);
    }
}
