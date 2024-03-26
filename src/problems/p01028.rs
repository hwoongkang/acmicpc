pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut first_line = lines.next().unwrap().split_ascii_whitespace();
    let mr: usize = first_line.next().unwrap().parse().unwrap();
    let mc: usize = first_line.next().unwrap().parse().unwrap();

    let diamonds: Vec<Vec<bool>> = lines
        .map(|line| line.chars().map(|ch| ch == '1').collect())
        .collect();

    let mut dp_from_up: Vec<Vec<usize>> = vec![vec![0; mc]; mr];
    let mut dp_from_down: Vec<Vec<usize>> = vec![vec![0; mc]; mr];

    for r in 0..mr {
        for c in 0..mc {
            let down = mr - r;
            let left = c + 1;
            let right = mc - c;
            let max_size = down.min(left).min(right);
            for size in 1..=max_size {
                let d = size - 1;
                let down = r + d;
                let left = c - d;
                let right = c + d;
                if diamonds[down][left] && diamonds[down][right] {
                    dp_from_up[r][c] = size;
                } else {
                    break;
                }
            }
        }
    }

    let mut ans = 0;

    for r in 0..mr {
        for c in 0..mc {
            let up = r / 2 + 1;
            let left = c + 1;
            let right = mc - c;
            let max_size = up.min(left).min(right);
            for size in 1..=max_size {
                let d = size - 1;
                let up = r - d;
                let left = c - d;
                let right = c + d;
                let across = r - 2 * d;
                if diamonds[up][left] && diamonds[up][right] {
                    dp_from_down[r][c] = size;
                    if dp_from_up[across][c] >= size {
                        ans = ans.max(size);
                    }
                } else {
                    break;
                }
            }
        }
    }

    ans.to_string()
}

#[cfg(test)]
mod p01028_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5 5
01100
01011
11111
01111
11111"
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "5 5
01100
00011
11111
01111
11111"
            .to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "4 4
0000
0000
0000
0000"
            .to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "3 6
111000
101111
111111"
            .to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }
}
