#[warn(dead_code)]
pub fn string_encoding(line: String) -> String {

    let mut output_text = String::new();
    let mut symbol = line.chars().next().unwrap();
    let mut count = 0;

    for list_symbol in line.chars() {
        if list_symbol == symbol {
            count += 1;
        }else {
            output_text.push(symbol);
            if count > 1 {
                output_text.push_str(&count.to_string());
            }

            symbol = list_symbol;
            count = 1;
        }
    }

    output_text.push(symbol);
    if count > 1 {
        output_text.push_str(&count.to_string());
    }

    return output_text;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grouping_symbol_word() {
        let text = String::from("hello");
        
        let expected_text = string_encoding(text);

        assert_eq!(String::from("hel2o"),  expected_text);
    }

    #[test]
    fn test_grouping_symbol_text() {
        let test_text = String::from("ssomEee   text heaRRe");

        let result = string_encoding(test_text);

        assert_eq!("s2omEe2 3text heaR2e", result);
    }
}