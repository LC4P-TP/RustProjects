use std::collections::HashSet;

fn remove_duplicate(str_1: String, str_2: String) -> String {
    let hash_set_1: HashSet<char> = str_1.chars().collect();
    let hash_set_2: HashSet<char> = str_2.chars().collect();

    let result = hash_set_1.difference(&hash_set_2).collect();

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicate_word() {
        let line_1 = String::from("word");
        let line_2 = String::from("word");

        let result = remove_duplicate(line_1, line_2);
        let expection = String::from("");

        assert_eq!(expection, result)
    }

    #[test]
    fn test_remove_duplicate_text() {
        let line_1 = String::from("first word");
        let line_2 = String::from("second the word");

        let result = remove_duplicate(line_1, line_2);
        let expection = String::from("if");

        assert_eq!(expection, result);
    }
}
