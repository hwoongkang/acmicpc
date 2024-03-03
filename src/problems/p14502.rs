#[derive(Clone)]
enum Tile {
    Empty,
    Wall,
    Virus,
}

impl Tile {
    fn from(s: &str) -> Self {
        match s {
            "0" => Self::Empty,
            "1" => Self::Wall,
            "2" => Self::Virus,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Lab {
    tiles: Vec<Vec<Tile>>,
}

impl Lab {
    fn from(s: &mut dyn Iterator<Item = &str>) -> Self {
        Self {
            tiles: s
                .map(|line| line.split_ascii_whitespace().map(Tile::from).collect())
                .collect(),
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }

    fn empty_positions(&self) -> Vec<(usize, usize)> {
        let (mr, mc) = self.size();
        (0..mr)
            .flat_map(|r| (0..mc).map(move |c| (r, c)))
            .filter_map(|(r, c)| match self.tiles[r][c] {
                Tile::Empty => Some((r, c)),
                _ => None,
            })
            .collect()
    }

    fn next(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let &(r, c) = pos;
        let (mr, mc) = self.size();
        if r > 0 {
            ans.push((r - 1, c))
        }
        if c > 0 {
            ans.push((r, c - 1))
        }
        if r + 1 < mr {
            ans.push((r + 1, c))
        }
        if c + 1 < mc {
            ans.push((r, c + 1))
        }
        ans.into_iter()
            .filter_map(|(r, c)| match self.tiles[r][c] {
                Tile::Empty => Some((r, c)),
                _ => None,
            })
            .collect()
    }

    fn simulate(&mut self, empty_positions: [(usize, usize); 3]) -> usize {
        for (r, c) in empty_positions {
            self.tiles[r][c] = Tile::Wall;
        }
        let (mr, mc) = self.size();
        for r in 0..mr {
            for c in 0..mc {
                match &self.tiles[r][c] {
                    Tile::Virus => {
                        let mut stack = vec![(r, c)];
                        self.tiles[r][c] = Tile::Virus;
                        while let Some((r, c)) = stack.pop() {
                            for (nr, nc) in self.next(&(r, c)) {
                                self.tiles[nr][nc] = Tile::Virus;
                                stack.push((nr, nc));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        self.safe_places()
    }

    fn safe_places(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::Empty => 1,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}

pub fn _solve(input: String) -> String {
    let lab = Lab::from(&mut input.lines().skip(1));
    let empty_positions = lab.empty_positions();

    let l = empty_positions.len();

    let mut ans = 0;
    for i in 0..l {
        for j in i + 1..l {
            for k in j + 1..l {
                ans = ans.max(lab.clone().simulate([
                    empty_positions[i],
                    empty_positions[j],
                    empty_positions[k],
                ]))
            }
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod p14502_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "7 7
2 0 0 0 1 1 0
0 0 1 0 1 2 0
0 1 1 0 1 0 0
0 1 0 0 0 0 0
0 0 0 0 0 1 1
0 1 0 0 0 0 0
0 1 0 0 0 0 0"
            .to_string();
        let output = "27".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4 6
0 0 0 0 0 0
1 0 0 0 0 2
1 1 1 0 0 2
0 0 0 0 0 2"
            .to_string();
        let output = "9".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "8 8
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
2 0 0 0 0 0 0 2
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0"
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }
}
