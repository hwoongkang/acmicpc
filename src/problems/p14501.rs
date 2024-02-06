pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let num_days: usize = lines.next().unwrap().parse().unwrap();
    let tasks = lines.map(|line| -> (usize, usize) {
        let mut words = line.split_ascii_whitespace();
        (
            words.next().unwrap().parse().unwrap(),
            words.next().unwrap().parse().unwrap(),
        )
    });

    let mut dp = vec![0; num_days + 1];

    let max_day = num_days + 1;

    for (today, (duration, reward)) in tasks.enumerate() {
        let ends = today + duration;
        let balance = dp[today];
        for day in ends..max_day {
            dp[day] = dp[day].max(balance + reward);
        }
    }
    dp.last().unwrap().to_string()
}

#[cfg(test)]
mod p14501_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "7
3 10
5 20
1 10
1 20
2 15
4 40
2 200"
            .to_string();
        let output = "45".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "10
1 1
1 2
1 3
1 4
1 5
1 6
1 7
1 8
1 9
1 10"
            .to_string();
        let output = "55".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "10
5 10
5 9
5 8
5 7
5 6
5 10
5 9
5 8
5 7
5 6"
        .to_string();
        let output = "20".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "10
5 50
4 40
3 30
2 20
1 10
1 10
2 20
3 30
4 40
5 50"
            .to_string();
        let output = "90".to_string();
        assert_eq!(_solve(input), output);
    }
}
