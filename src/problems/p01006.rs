pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let test_cases: usize = lines.next().unwrap().parse().unwrap();
    let mut answers = vec![];
    for _ in 0..test_cases {
        answers.push(_testcase(lines));
    }

    answers.join("\n")
}

fn _testcase(lines: &mut std::str::Lines) -> String {
    let size: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let enemies: Vec<Vec<usize>> = lines
        .take(2)
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect()
        })
        .collect();
    // type_a: r
    // type_b: L
    // type_c: |

    let max = 1_000_000_000;
    let mut ans = max;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; enemies.len()]; 3];

    dp[0][0] = 1;
    dp[0][1] = 1;
    dp[0][2] = if dp[0][0] + dp[0][1] <= size { 1 } else { max };

    dp[1][2] = if (enemies[0][0] + enemies[0][1] <= size) && (enemies[1][0] + enemies[1][1] <= size)
    {
        2
    } else {
        max
    };
    for col in 1..enemies.len() {}

    String::new()
}

#[cfg(test)]
mod p01006_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "1
8 100
70 60 55 43 57 60 44 50
58 40 47 90 45 52 80 40"
            .to_string();
        let output = "11".to_string();
        assert_eq!(_solve(input), output);
    }
}
