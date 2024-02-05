pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let time: Vec<u16> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    let h = time[0];
    let m = time[1];
    let elapsed: u16 = lines.next().unwrap().parse().unwrap();

    let m = m + elapsed;
    let carry = m / 60;
    let m = m % 60;

    let h = (h + carry) % 24;

    format!("{} {}", h, m)
}

#[cfg(test)]
mod p02525_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "14 30
20"
        .to_string();
        let output = "14 50".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "17 40
80"
        .to_string();
        let output = "19 0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "23 48
25"
        .to_string();
        let output = "0 13".to_string();
        assert_eq!(_solve(input), output);
    }
}
