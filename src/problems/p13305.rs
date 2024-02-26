pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let distances: Vec<usize> = lines
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    let costs: Vec<usize> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    let mut min = usize::MAX;
    let mut ans = 0;
    for (dist, &cost) in distances.iter().zip(costs.iter()) {
        min = min.min(cost);
        ans += min * dist;
    }
    ans.to_string()
}

#[cfg(test)]
mod p13305_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "4
2 3 1
5 2 4 1"
            .to_string();
        let output = "18".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4
3 3 4
1 1 1 1"
            .to_string();
        let output = "10".to_string();
        assert_eq!(_solve(input), output);
    }
}
