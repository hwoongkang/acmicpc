pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let first: Vec<i32> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    let ans = first[0] * first[1];
    lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse::<i32>().unwrap())
        .map(|n| n - ans)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod p02845_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "1 10
10 10 10 10 10"
            .to_string();
        let output = "0 0 0 0 0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "5 20
99 101 1000 0 97"
            .to_string();
        let output = "-1 1 900 -100 -3".to_string();
        assert_eq!(_solve(input), output);
    }
}
