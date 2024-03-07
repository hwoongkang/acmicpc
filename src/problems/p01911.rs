pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();

    let mut line = lines.next().unwrap().split_ascii_whitespace();

    let plank_len: usize = line.nth(1).unwrap().parse().unwrap();

    let mut puddles: Vec<(usize, usize)> = lines
        .map(|line| {
            let mut line = line.split_ascii_whitespace();
            (
                line.next().unwrap().parse().unwrap(),
                line.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    puddles.sort();

    let mut prev = 0;

    let mut count = 0;

    for puddle in puddles {
        let start = prev.max(puddle.0);
        if start >= puddle.1 {
            prev = start;
        } else {
            let puddle_len = puddle.1 - start;
            let needed = (puddle_len + (plank_len - 1)) / plank_len;
            count += needed;
            prev = start + needed * plank_len;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod p01911_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3 3
1 6
13 17
8 12"
            .to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }
}
