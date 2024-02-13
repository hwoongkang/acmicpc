pub fn _solve(input: String) -> String {
    let mut words = input.split_ascii_whitespace();
    let a = words.next().unwrap();
    let b = words.next().unwrap();
    let (a, b) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    let al = a.len();
    let bl = b.len();
    let diff = bl - al;
    (0..=diff)
        .map(|d| {
            a.chars()
                .zip(b[d..].chars())
                .map(|(a, b)| u8::from(a != b))
                .sum::<u8>()
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod p01120_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "adaabc aababbc".to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "hello xello".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "koder topcoder".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "abc topabcoder".to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_5() {
        let input = "giorgi igroig".to_string();
        let output = "6".to_string();
        assert_eq!(_solve(input), output);
    }
}
