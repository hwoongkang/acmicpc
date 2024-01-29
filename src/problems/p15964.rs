fn _at(a: i64, b: i64) -> i64 {
    a * a - b * b
}
pub fn _solve(input: String) -> String {
    let mut words = input.split_whitespace();
    let a: i64 = words.next().unwrap().parse().unwrap();
    let b: i64 = words.next().unwrap().parse().unwrap();

    _at(a, b).to_string()
}

#[cfg(test)]
mod p15964_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "4 3".to_string();
        let output = "7".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_2() {
        let input = "3 4".to_string();
        let output = "-7".to_string();
        assert_eq!(_solve(input), output);
    }
}
