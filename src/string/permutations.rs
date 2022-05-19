use std::collections::*;
use itertools::Itertools;


pub fn string_training() {
    let string = String::from("TOR");
    
    print!("{:?}", permutations(string))
}

fn permutations(some_string: String) -> Vec<String> {
    let basis: Vec<char> = some_string.chars().collect();
    let mut result: Vec<String> = vec![];

    for perm in basis.iter().permutations(basis.len()).unique() {
        result.push(String::from_iter(perm))
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_string() {
        let test_string = String::from("ABC");

        let rusult = permutations(test_string);
        let expected: Vec<String> = vec!["ABC".to_string(), "ACB".to_string(), "BAC".to_string(),
        "BCA".to_string(), "CAB".to_string(), "CBA".to_string()];
        
        assert_eq!(expected, rusult);
    }
}
