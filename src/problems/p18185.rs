pub fn _solve(input: String) -> String {
    let mut prev = (0, 0, 0);
    let mut count = vec![];
    for mut num in input
        .lines()
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse::<usize>().unwrap())
    {
        let mut now = (0, 0, 0);
        if prev.0 >= num {
            now = (0, num, 0);
            count.push(now);
            prev = now;
            continue;
        } else {
            num -= prev.0;
            now.1 = prev.0;
        }
        if prev.1 >= num {
            now.2 = num;
            count.push(now);
            prev = now;
            continue;
        } else {
            num -= prev.1;
            now.2 = prev.1;
        }
        now.0 = num;
        count.push(now);
        prev = now;
    }

    count
        .into_iter()
        .map(|(a, b, c)| 3 * a + 2 * b + 2 * c)
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod p18185_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
1 0 1"
            .to_string();
        let output = "6".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "5
1 1 1 0 2"
            .to_string();
        let output = "13".to_string();
        assert_eq!(_solve(input), output);
    }
}
