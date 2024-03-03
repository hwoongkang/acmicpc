use std::collections::VecDeque;

struct Shark {
    size: u8,
    stomach: u8,
    pos: (usize, usize),
}

impl Shark {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            size: 2,
            stomach: 0,
            pos,
        }
    }

    fn can_eat(&self, fish_size: u8) -> bool {
        self.size > fish_size
    }

    fn can_pass(&self, fish_size: u8) -> bool {
        self.size >= fish_size
    }

    fn eat(&mut self) {
        self.stomach += 1;
        if self.size == self.stomach {
            self.size += 1;
            self.stomach = 0;
        }
    }
}
enum Tile {
    Empty,
    Fish(u8),
}

struct FishTank {
    tiles: Vec<Vec<Tile>>,
}

impl FishTank {
    fn from(s: &mut dyn Iterator<Item = &str>) -> (Self, (usize, usize)) {
        let mut shark_pos = (0, 0);
        (
            Self {
                tiles: s
                    .enumerate()
                    .map(|(r, line)| {
                        line.split_ascii_whitespace()
                            .enumerate()
                            .map(|(c, word)| match word {
                                "0" => Tile::Empty,
                                "9" => {
                                    shark_pos = (r, c);
                                    Tile::Empty
                                }
                                n => Tile::Fish(n.parse().unwrap()),
                            })
                            .collect()
                    })
                    .collect(),
            },
            shark_pos,
        )
    }

    fn size(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }

    fn next_pos(
        &self,
        pos: (usize, usize),
        shark: &Shark,
        visited: &[Vec<bool>],
    ) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let &(r, c) = &pos;
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
            .filter_map(|(r, c)| if visited[r][c] { None } else { Some((r, c)) })
            .filter_map(|(r, c)| match self.tiles[r][c] {
                Tile::Empty => Some((r, c)),
                Tile::Fish(fish_size) => {
                    if shark.can_pass(fish_size) {
                        Some((r, c))
                    } else {
                        None
                    }
                }
            })
            .collect()
    }

    fn tick(&mut self, shark: &mut Shark) -> Option<usize> {
        let (mr, mc) = self.size();
        let mut visited = vec![vec![false; mc]; mr];
        visited[shark.pos.0][shark.pos.1] = true;
        let mut queue = VecDeque::from([(shark.pos, 0)]);

        let mut found_dist = usize::MAX;
        let mut candidates: Vec<(usize, usize)> = vec![];

        while let Some((pos, dist)) = queue.pop_front() {
            let dist = dist + 1;
            if dist > found_dist {
                break;
            }
            for (nr, nc) in self.next_pos(pos, &shark, &visited) {
                match self.tiles[nr][nc] {
                    Tile::Fish(size) => {
                        if shark.can_eat(size) {
                            found_dist = dist;
                            candidates.push((nr, nc));
                            continue;
                        }
                    }
                    Tile::Empty => {}
                }
                if dist == found_dist {
                    continue;
                }
                visited[nr][nc] = true;
                queue.push_back(((nr, nc), dist));
            }
        }
        if candidates.is_empty() {
            None
        } else {
            candidates.sort();
            let fish = candidates[0];
            shark.eat();
            shark.pos = fish;
            self.tiles[fish.0][fish.1] = Tile::Empty;
            Some(found_dist)
        }
    }
}
pub fn _solve(input: String) -> String {
    let (mut tank, shark_pos) = FishTank::from(&mut input.lines().skip(1));
    let mut shark = Shark::new(shark_pos);
    let mut ans = 0;
    while let Some(time) = tank.tick(&mut shark) {
        ans += time;
    }
    ans.to_string()
}

#[cfg(test)]
mod p16236_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
0 0 0
0 0 0
0 9 0"
            .to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "3
0 0 1
0 0 0
0 9 0"
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "4
4 3 2 1
0 0 0 0
0 0 9 0
1 2 3 4"
            .to_string();
        let output = "14".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "6
5 4 3 2 3 4
4 3 2 3 4 5
3 2 9 5 6 6
2 1 2 3 4 5
3 2 1 6 5 4
6 6 6 6 6 6"
            .to_string();
        let output = "60".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_5() {
        let input = "6
6 0 6 0 6 1
0 0 0 0 0 2
2 3 4 5 6 6
0 0 0 0 0 2
0 2 0 0 0 0
3 9 3 0 0 1"
            .to_string();
        let output = "48".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_6() {
        let input = "6
1 1 1 1 1 1
2 2 6 2 2 3
2 2 5 2 2 3
2 2 2 4 6 3
0 0 0 0 0 6
0 0 0 0 0 9"
            .to_string();
        let output = "39".to_string();
        assert_eq!(_solve(input), output);
    }
}
