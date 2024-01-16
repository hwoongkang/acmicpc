use std::collections::VecDeque;
pub fn _solve(input: String) -> String {
    let mut start = (0, 0);
    let mut map: Vec<Vec<i32>> = input
        .lines()
        .skip(1)
        .enumerate()
        .map(|(r, line)| {
            line.split_whitespace()
                .enumerate()
                .map(|(c, word)| {
                    if word == "0" {
                        0
                    } else if word == "2" {
                        start = (r, c);
                        -1
                    } else {
                        -1
                    }
                })
                .collect()
        })
        .collect();

    let mut queue = VecDeque::from([(start, 0)]);

    map[start.0][start.1] = 0;

    let max_r = map.len();
    let max_c = map[1].len();

    let next_tiles = |(r, c): (usize, usize)| {
        let mut ans = vec![];
        if r > 0 {
            ans.push((r - 1, c));
        }
        if c > 0 {
            ans.push((r, c - 1));
        }
        if r + 1 < max_r {
            ans.push((r + 1, c));
        }
        if c + 1 < max_c {
            ans.push((r, c + 1))
        }
        ans
    };

    while let Some((pos, dist)) = queue.pop_front() {
        let dist = dist + 1;
        for (r, c) in next_tiles(pos) {
            if map[r][c] >= 0 {
                continue;
            } else {
                map[r][c] = dist;
                queue.push_back(((r, c), dist));
            }
        }
    }

    map.iter()
        .map(|row| {
            row.iter()
                .map(|dist| dist.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod p14940_test {
    use super::*;

    #[test]
    fn test() {
        let input = "15 15
2 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 1 1 1 1 1
1 1 1 1 1 1 1 1 1 1 0 0 0 0 1
1 1 1 1 1 1 1 1 1 1 0 1 1 1 1
1 1 1 1 1 1 1 1 1 1 0 1 0 0 0
1 1 1 1 1 1 1 1 1 1 0 1 1 1 1"
            .to_string();
        let output = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14
1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
3 4 5 6 7 8 9 10 11 12 13 14 15 16 17
4 5 6 7 8 9 10 11 12 13 14 15 16 17 18
5 6 7 8 9 10 11 12 13 14 15 16 17 18 19
6 7 8 9 10 11 12 13 14 15 16 17 18 19 20
7 8 9 10 11 12 13 14 15 16 17 18 19 20 21
8 9 10 11 12 13 14 15 16 17 18 19 20 21 22
9 10 11 12 13 14 15 16 17 18 19 20 21 22 23
10 11 12 13 14 15 16 17 18 19 20 21 22 23 24
11 12 13 14 15 16 17 18 19 20 0 0 0 0 25
12 13 14 15 16 17 18 19 20 21 0 29 28 27 26
13 14 15 16 17 18 19 20 21 22 0 30 0 0 0
14 15 16 17 18 19 20 21 22 23 0 31 32 33 34"
            .to_string();
        assert_eq!(_solve(input), output);
    }
}
