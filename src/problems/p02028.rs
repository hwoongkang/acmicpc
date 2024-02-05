pub fn _solve(input: String) -> String {
    input
        .lines()
        .skip(1)
        .map(|w| {
            let len = w.len();
            let n: usize = w.parse().unwrap();
            let pow = (0..len).fold(1, |acc, _| acc * 10);
            (n * n) % pow == n
        })
        .map(|b| if b { "YES" } else { "NO" })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod p02028_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "4
1
6
76
89"
        .to_string();
        let output = "YES
YES
YES
NO"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
