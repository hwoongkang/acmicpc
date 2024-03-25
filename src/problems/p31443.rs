const NUM: usize = 1_000_000_007;
fn blocks(n: usize, m: usize) -> (usize, usize) {
    if n % 3 == 0 {
        ((n / 3) * m, 0)
    } else if m % 3 == 0 {
        (n * (m / 3), 0)
    } else {
        match (n % 3, m % 3) {
            (1, 1) => {
                let row = n / 3;
                let mut threes = row * m;
                let col = m / 3;
                threes += col - 1;
                (threes, 2)
            }
            (1, 2) => {
                // 4, 5
                let bulk = n / 3;
                let mut threes = bulk * m;
                let col = m / 3;
                threes += col;
                (threes, 1)
            }
            (2, 1) => {
                // 5, 4
                let bulk = m / 3;
                let mut threes = bulk * n;
                let col = n / 3;
                threes += col;
                (threes, 1)
            }
            (2, 2) => {
                // 5, 5
                let bulk = n / 3;
                let mut threes = bulk * m;
                let col = m / 3;
                threes += col * 2;
                (threes, 2)
            }
            _ => unreachable!(),
        }
    }
}
pub fn _solve(input: String) -> String {
    let mut words = input.split_ascii_whitespace();
    let mr: usize = words.next().unwrap().parse().unwrap();
    let mc: usize = words.next().unwrap().parse().unwrap();

    if mr == 1 && mc == 1 {
        return "1".to_string();
    }

    let (threes, twos) = blocks(mr, mc);

    let mut ans = 1;

    for _ in 0..threes {
        ans *= 3;
        ans %= NUM;
    }
    for _ in 0..twos {
        ans *= 2;
        ans %= NUM;
    }

    ans.to_string()
}

#[cfg(test)]
mod p31443_tests {
    use super::*;

    #[test]
    fn test_case_0() {
        let input = "1 1".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_1() {
        let input = "2 2".to_string();
        let output = "4".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "1 5".to_string();
        let output = "6".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "1000 1000".to_string();
        let output = "206602388".to_string();
        assert_eq!(_solve(input), output);
    }
}
