pub fn _solve(input: String) -> String {
    let mut start = (0, 0);
    let mut map: Vec<Vec<char>> = input
        .lines()
        .skip(1)
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, char)| {
                    if char == 'I' {
                        start = (r, c);
                    }
                    char
                })
                .collect()
        })
        .collect();
    map[start.0][start.1] = 'X';
    let mut stack = vec![start];

    let max_r = map.len();
    let max_c = map[0].len();

    let mut ans = 0;
    while let Some((r, c)) = stack.pop() {
        let mut nexts = vec![];
        if r > 0 {
            nexts.push((r - 1, c));
        }
        if c > 0 {
            nexts.push((r, c - 1))
        }
        if r + 1 < max_r {
            nexts.push((r + 1, c));
        }
        if c + 1 < max_c {
            nexts.push((r, c + 1));
        }
        for (r, c) in nexts.into_iter() {
            if map[r][c] == 'X' {
                continue;
            }
            if map[r][c] == 'P' {
                ans += 1;
            }
            map[r][c] = 'X';
            stack.push((r, c));
        }
    }
    if ans == 0 {
        "TT".to_string()
    } else {
        ans.to_string()
    }
}

#[cfg(test)]
mod p21736_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "3 5
OOOPO
OIOOX
OOOXP"
            .to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_2() {
        let input = "3 3
IOX
OXP
XPP"
        .to_string();
        let output = "TT".to_string();
        assert_eq!(_solve(input), output);
    }
}
