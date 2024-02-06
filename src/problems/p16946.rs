struct _Map {
    tiles: Vec<Vec<usize>>,
}

impl _Map {
    fn _from(input: String) -> Self {
        let tiles = input
            .lines()
            .skip(1)
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        '0' => 0,
                        '1' => 1,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Self { tiles }
    }

    fn _size(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }
    fn _next(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let (max_r, max_c) = self._size();
        let (r, c) = pos;
        if r > 0 {
            ans.push((r - 1, c));
        }
        if c > 0 {
            ans.push((r, c - 1))
        }
        if r + 1 < max_r {
            ans.push((r + 1, c))
        }
        if c + 1 < max_c {
            ans.push((r, c + 1))
        }
        ans
    }

    fn _clustering(&mut self) -> Vec<u32> {
        let mut block = 1;
        let (max_r, max_c) = self._size();
        let mut sizes = vec![0, 0];
        for r in 0..max_r {
            for c in 0..max_c {
                if self.tiles[r][c] != 0 {
                    continue;
                }
                block += 1;
                let mut stack = vec![(r, c)];
                self.tiles[r][c] = block;
                sizes.push(1);
                while let Some(pos) = stack.pop() {
                    for (r, c) in self._next(pos) {
                        if self.tiles[r][c] == 0 {
                            self.tiles[r][c] = block;
                            sizes[block] += 1;
                            stack.push((r, c))
                        }
                    }
                }
            }
        }

        sizes
    }
}

pub fn _solve(input: String) -> String {
    let mut map = _Map::_from(input);
    let sizes = map._clustering();

    let ans: String = map
        .tiles
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, tile)| match tile {
                    1 => {
                        let mut ans = 1;
                        let mut seen: Vec<bool> = sizes.iter().map(|_| false).collect();
                        for (r, c) in map._next((r, c)) {
                            match map.tiles[r][c] {
                                0 | 1 => {}
                                n => {
                                    if !seen[n] {
                                        ans += sizes[n];
                                        seen[n] = true;
                                    }
                                }
                            }
                        }
                        ans %= 10;
                        char::from_digit(ans, 10).unwrap()
                    }
                    _ => '0',
                })
                .collect()
        })
        .collect::<Vec<String>>()
        .join("\n");
    ans
}

#[cfg(test)]
mod p16946_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3 3
101
010
101"
        .to_string();
        let output = "303
050
303"
        .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4 5
11001
00111
01010
10101"
            .to_string();
        let output = "46003
00732
06040
50403"
            .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "1 11
00000000010"
            .to_string();
        let output = "00000000010".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "3 3
000
010
000
101"
        .to_string();
        let output = "000
000
000
000"
        .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_5() {
        let input = "6 6
101010
000000
101010
000000
101010
000000"
            .to_string();
        let output = "808080
000000
808080
000000
808080
000000"
            .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_6() {
        let input = "5 5
01111
10100
01000
10111
00000"
            .to_string();
        let output = "03166
40700
04000
80222
00000"
            .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_7() {
        let input = "3 5
01000
10000
00000"
            .to_string();
        let output = "04000
40000
00000"
            .to_string();
        assert_eq!(_solve(input), output);
    }
}
