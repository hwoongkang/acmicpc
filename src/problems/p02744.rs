pub fn _solve(input: String) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_lowercase() {
                ch.to_ascii_uppercase()
            } else {
                ch.to_ascii_lowercase()
            }
        })
        .collect()
}

#[cfg(test)]
mod p02744_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "WrongAnswer".to_string();
        let output = "wRONGaNSWER".to_string();
        assert_eq!(_solve(input), output);
    }
}
