pub fn _solve(input: String) -> String {
    input
        .split_whitespace()
        .map(|w| w.parse::<u64>().unwrap())
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod p11382_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "77 77 7777".to_string();
        let output = "7931".to_string();
        assert_eq!(_solve(input), output);
    }
}
