const ZERO: &str = "###
#.#
#.#
#.#
###";

const ONE: &str = "..#
..#
..#
..#
..#";
const TWO: &str = "###
..#
###
#..
###";
const THREE: &str = "###
..#
###
..#
###";
const FOUR: &str = "#.#
#.#
###
..#
..#";
const FIVE: &str = "###
#..
###
..#
###";
const SIX: &str = "###
#..
###
#.#
###";
const SEVEN: &str = "###
..#
..#
..#
..#";
const EIGHT: &str = "###
#.#
###
#.#
###";
const NINE: &str = "###
#.#
###
..#
###";

const NUMS: [&str; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

fn digit_to_str(num: usize) -> &'static str {
    NUMS[num]
}

fn str_to_digit(str: &str) -> Vec<Vec<char>> {
    str.lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn matches(big: &[Vec<char>], small: &[Vec<char>]) -> bool {
    fn allowed(big: char, small: char) -> bool {
        matches!((big, small), ('.', '.') | ('#', '#') | ('#', '.'))
    }

    for r in 0..5 {
        for c in 0..3 {
            if !allowed(big[r][c], small[r][c]) {
                return false;
            }
        }
    }
    true
}

pub fn _solve(input: String) -> String {
    let ans = inner(input);
    if ans == -1.0 {
        "-1".to_string()
    } else {
        ans.to_string()
    }
}

fn inner(input: String) -> f64 {
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let chars: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let digits: Vec<Vec<Vec<char>>> = (0..n)
        .map(|i| {
            let start = 4 * i;
            (0..5)
                .map(|r| (0..3).map(|c| chars[r][start + c]).collect())
                .collect()
        })
        .collect();

    let mut candidates: Vec<Vec<usize>> = vec![];
    for digit in digits {
        let mut now = vec![];
        for (n, num) in NUMS.iter().enumerate() {
            let num = &str_to_digit(num);
            if matches(num, &digit) {
                now.push(n);
            }
        }

        candidates.push(now);
    }
    for c in candidates.iter() {
        if c.is_empty() {
            return -1.0;
        }
    }
    let lens = candidates.iter().map(|c| c.len());
    let total_cases: usize = lens.product();
    let mut total = 0;
    let mut power = 1;
    for c in candidates.iter().rev() {
        let cases = total_cases / (c.len());
        for n in c.iter() {
            total += power * n * cases;
        }
        power *= 10;
    }
    (total as f64) / (total_cases as f64)
}

#[cfg(test)]
mod p01089_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "1
###
#.#
###
#.#
###"
        .to_string();
        let input = inner(input);
        let output = 8.0;
        assert!((input - output).abs() < 10e-5)
    }

    #[test]
    fn test_case_2() {
        let input = "2
###.###
#.#.#.#
#.#.###
#.#...#
###.###"
            .to_string();
        let input = inner(input);
        let output = 48.5;
        assert!((input - output).abs() < 10e-5)
    }

    #[test]
    fn test_case_3() {
        let input = "2
.......
.......
.......
.......
......."
            .to_string();
        let input = inner(input);
        let output = 49.5;
        assert!((input - output).abs() < 10e-5)
    }

    #[test]
    fn test_case_4() {
        let input = "1
...
.#.
...
...
..."
        .to_string();
        let input = inner(input);
        let output = -1.0;
        assert!((input - output).abs() < 10e-5)
    }
}
