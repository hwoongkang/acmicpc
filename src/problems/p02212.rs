pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let _n: usize = lines.next().unwrap().trim().parse().unwrap();
    let k: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut sensors: Vec<i64> = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    sensors.sort();
    let mut edges: Vec<i64> = sensors.windows(2).map(|w| w[1] - w[0]).collect();

    edges.sort();
    let mut ans = edges.iter().sum::<i64>();
    ans -= edges.iter().rev().take(k - 1).sum::<i64>();

    ans.to_string()
}

#[cfg(test)]
mod p02212_tests {
    use super::*;

    #[test]
    fn test_case_2() {
        let input = "6
2
1 6 9 3 6 7"
            .to_string();
        // 1 3 6 6 7 9
        // 2 3 0 1 2
        // 1 -> 8
        // 2 -> 5

        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_1() {
        let input = "10
5
20 3 14 6 7 8 18 10 12 15"
            .to_string();
        // 3 6 7 8 10 12 14 15 18 20
        // 3 1 1 2 2 2 1 3 2
        // 17
        // 14
        // 11
        // 9
        // 7
        let output = "7".to_string();
        assert_eq!(_solve(input), output);
    }
}
