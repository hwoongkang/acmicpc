pub fn _solve(input: String) -> String {
    input.trim().parse::<u8>().unwrap().count_ones().to_string()
}

#[cfg(test)]
mod p01094_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "23".to_string();
        let output = "4".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "64".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "32".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "48".to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }
}
