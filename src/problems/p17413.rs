fn flip(input: String) -> String {
    input
        .split_ascii_whitespace()
        .map(|str| str.chars().rev().collect())
        .collect::<Vec<String>>()
        .join(" ")
}
pub fn _solve(input: String) -> String {
    let mut parsed: Vec<(String, bool)> = vec![];
    let mut str = String::new();

    for char in input.chars() {
        match char {
            '<' => {
                if !str.is_empty() {
                    parsed.push((str, false));
                    str = String::new();
                }
                str.push(char);
            }
            '>' => {
                str.push(char);
                parsed.push((str, true));
                str = String::new();
            }
            char => {
                str.push(char);
            }
        }
    }
    if !str.is_empty() {
        parsed.push((str, false));
    }

    parsed
        .into_iter()
        .map(|(word, is_tag)| if is_tag { word } else { flip(word) })
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod p17413_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "<ab cd>ef gh<ij kl>".to_string();
        let output = "<ab cd>fe hg<ij kl>".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "<int><max>2147483647<long long><max>9223372036854775807".to_string();
        let output = "<int><max>7463847412<long long><max>7085774586302733229".to_string();
        assert_eq!(_solve(input), output);
    }
}
