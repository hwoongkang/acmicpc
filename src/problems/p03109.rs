enum Tile {
    Empty,
    Full,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn from(s: &mut dyn Iterator<Item = &str>) -> Self {
        Self {
            tiles: s
                .map(|line| {
                    line.chars()
                        .map(|ch| match ch {
                            '.' => Tile::Empty,
                            'x' => Tile::Full,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn next(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mr = self.tiles.len();

        (0..=2)
            .filter_map(move |dr| {
                let r = pos.0 + dr;
                let c = pos.1 + 1;
                if r == 0 || r == mr + 1 {
                    None
                } else {
                    Some((r - 1, c))
                }
            })
            .filter_map(|(r, c)| {
                if let Tile::Empty = self.tiles[r][c] {
                    Some((r, c))
                } else {
                    None
                }
            })
            .collect()
    }

    fn solve(&mut self) -> usize {
        let mut count = 0;

        let mr = self.tiles.len();

        fn dfs(map: &mut Map, pos: (usize, usize), count: &mut usize) -> bool {
            if pos.1 == map.tiles[0].len() - 1 {
                *count += 1;
                true
            } else {
                for (nr, nc) in map.next(&pos) {
                    map.tiles[pos.0][pos.1] = Tile::Full;
                    if dfs(map, (nr, nc), count) {
                        return true;
                    }
                }
                false
            }
        }

        for r in 0..mr {
            dfs(self, (r, 0), &mut count);
        }

        count
    }
}

pub fn _solve(input: String) -> String {
    let mut map = Map::from(&mut input.lines().skip(1));
    map.solve().to_string()
}

#[cfg(test)]
mod p03109_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5 5
.xx..
..x..
.....
...x.
...x."
            .to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "6 10
..x.......
.....x....
.x....x...
...x...xx.
..........
....x....."
            .to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }
}
