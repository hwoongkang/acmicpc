pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let num_testcases: usize = lines.next().unwrap().parse().unwrap();
    (0..num_testcases)
        .map(|_| _testcase(lines))
        .collect::<Vec<_>>()
        .join("\n")
}

type _RowConfig = usize;

fn _num_students(mut config: _RowConfig) -> usize {
    let mut ans = 0;
    while config > 0 {
        ans += config & 1;
        config >>= 1;
    }
    ans
}

fn _cannot_cheat(lhs: _RowConfig, rhs: _RowConfig) -> bool {
    (lhs & (rhs << 1) == 0) && ((lhs & rhs >> 1) == 0)
}

fn _can_sit(config: _RowConfig, seats: _RowConfig) -> bool {
    (!seats) & config == 0
}

fn _parse_seat(line: &str) -> _RowConfig {
    line.chars()
        .map(|ch| match ch {
            'x' => 0,
            _ => 1,
        })
        .fold(0, |acc, now| (acc << 1) | now)
}

fn _testcase(lines: &mut std::str::Lines) -> String {
    let mut dp = [[0usize; 1024]; 11];
    let mut first_line = lines.next().unwrap().split_whitespace();
    let max_r: usize = first_line.next().unwrap().parse().unwrap();
    let max_c: usize = first_line.next().unwrap().parse().unwrap();

    let max_c = 1 << max_c;

    let seats: Vec<_RowConfig> = lines.take(max_r).map(_parse_seat).collect();

    for r in 0..max_r {
        let seat = seats[r];

        for curr in 0..max_c {
            if !_cannot_cheat(curr, curr) || !_can_sit(curr, seat) {
                dp[r + 1][curr] = 0;
            } else {
                let mut now = 0;
                for prev in 0..max_c {
                    let acc = dp[r][prev];
                    if _cannot_cheat(curr, prev) {
                        now = now.max(acc + _num_students(curr));
                    }
                }
                dp[r + 1][curr] = now;
            }
        }
    }

    dp[max_r].iter().max().unwrap().to_string()
}

#[cfg(test)]
mod p01014_tests {
    use super::*;

    #[test]
    fn test_whole() {
        let input = "4
2 3
...
...
2 3
x.x
xxx
2 3
x.x
x.x
10 10
....x.....
..........
..........
..x.......
..........
x...x.x...
.........x
...x......
........x.
.x...x...."
            .to_string();
        let output = "4
1
2
46"
        .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_parsing() {
        let input = ".x.";
        assert_eq!(_parse_seat(input), 5);
    }

    #[test]
    fn test_seats() {
        let input = "xxx";
        let seat = _parse_seat(input);
        let config = 0;
        assert!(_can_sit(config, seat))
    }
    #[test]
    fn test_case_1() {
        let input = "2 3
...
..."
        .to_string();
        let output = "4".to_string();
        assert_eq!(_testcase(&mut input.lines()), output);
    }

    #[test]
    fn test_case_2() {
        let input = "2 3
x.x
xxx"
        .to_string();
        let output = "1".to_string();
        assert_eq!(_testcase(&mut input.lines()), output);
    }
}
