pub fn _solve(input: String) -> String {
    let product = input
        .split_ascii_whitespace()
        .map(|w| -> u32 { w.parse().unwrap() })
        .take(2)
        .product::<u32>();
    (product - 1).to_string()
}

#[cfg(test)]
mod p02163_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "2 2".to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "1 1".to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }
}
