pub fn _solve(input: String) -> String {
    let mut words = input.split_ascii_whitespace();
    let a: usize = words.next().unwrap().parse().unwrap();
    let mut b: usize = words.next().unwrap().parse().unwrap();
    let mut count = 0;
    while a < b {
        count += 1;
        if b % 2 == 0 {
            b /= 2;
        } else if b % 10 != 1 {
            return "-1".to_string();
        } else {
            b /= 10;
        }
    }
    if a == b {
        (count + 1).to_string()
    } else {
        "-1".to_string()
    }
}

#[cfg(test)]
mod p16953_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "2 162".to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4 42".to_string();
        let output = "-1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "100 40021".to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }
}
