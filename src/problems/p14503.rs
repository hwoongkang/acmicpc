enum Dir {
    North,
    South,
    East,
    West,
}

type Pos = (usize, usize);

enum Tile {
    Clean,
    Dirty,
    Wall,
}

struct CodeZero {
    facing: Dir,
    pos: Pos,
    map: Vec<Vec<Tile>>,
    count: usize,
}

use Dir::*;
use Tile::*;

impl CodeZero {
    fn from(lines: &mut dyn Iterator<Item = &str>) -> Self {
        let mut line = lines.nth(1).unwrap().split_ascii_whitespace();
        let r: usize = line.next().unwrap().parse().unwrap();
        let c: usize = line.next().unwrap().parse().unwrap();
        let pos = (r, c);
        let facing = match line.next().unwrap() {
            "0" => North,
            "1" => East,
            "2" => South,
            "3" => West,
            _ => unreachable!(),
        };
        let map: Vec<Vec<Tile>> = lines
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|word| match word {
                        "0" => Dirty,
                        "1" => Wall,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        Self {
            facing,
            pos,
            map,
            count: 0,
        }
    }

    fn map_size(&self) -> Pos {
        (self.map.len(), self.map[0].len())
    }

    fn go_forward(&mut self) {
        let (r, c) = self.pos;
        match self.facing {
            North => self.pos = (r - 1, c),
            South => self.pos = (r + 1, c),
            East => self.pos = (r, c + 1),
            West => self.pos = (r, c - 1),
        }
    }

    fn go_backward(&mut self) -> bool {
        let backward = match self.facing {
            North => self.south(),
            South => self.north(),
            West => self.east(),
            East => self.west(),
        };
        if let Some(pos) = backward {
            self.pos = pos;
            true
        } else {
            false
        }
    }

    fn turn_left(&mut self) {
        self.facing = match self.facing {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    fn north(&self) -> Option<Pos> {
        let (r, c) = self.pos;
        if r > 0 {
            let (r, c) = (r - 1, c);
            if let Tile::Wall = self.map[r][c] {
                None
            } else {
                Some((r, c))
            }
        } else {
            None
        }
    }

    fn west(&self) -> Option<Pos> {
        let (r, c) = self.pos;
        if c > 0 {
            let (r, c) = (r, c - 1);
            if let Tile::Wall = self.map[r][c] {
                None
            } else {
                Some((r, c))
            }
        } else {
            None
        }
    }

    fn south(&self) -> Option<Pos> {
        let (r, c) = self.pos;
        let (mr, _) = self.map_size();
        if r + 1 < mr {
            let (r, c) = (r + 1, c);
            if let Tile::Wall = self.map[r][c] {
                None
            } else {
                Some((r, c))
            }
        } else {
            None
        }
    }

    fn east(&self) -> Option<Pos> {
        let (r, c) = self.pos;
        let (_, mc) = self.map_size();
        if c + 1 < mc {
            let (r, c) = (r, c + 1);
            if let Tile::Wall = self.map[r][c] {
                None
            } else {
                Some((r, c))
            }
        } else {
            None
        }
    }

    fn phase_1(&mut self) -> bool {
        let (r, c) = self.pos;
        match self.map[r][c] {
            Dirty => {
                self.map[r][c] = Clean;
                self.count += 1;
                true
            }
            Clean => false,
            Wall => unreachable!(),
        }
    }

    fn scan_forward(&self) -> bool {
        let forward = match self.facing {
            North => self.north(),
            South => self.south(),
            East => self.east(),
            West => self.west(),
        };

        if let Some(pos) = forward {
            let tile = &self.map[pos.0][pos.1];
            matches!(tile, Tile::Dirty)
        } else {
            false
        }
    }

    fn phase_3(&mut self) -> bool {
        for _ in 0..4 {
            self.turn_left();
            if self.scan_forward() {
                self.go_forward();
                return true;
            }
        }
        false
    }

    fn phase_2(&mut self) -> bool {
        self.go_backward()
    }

    fn tick(&mut self) -> bool {
        self.phase_1() || self.phase_3() || self.phase_2()
    }

    fn start(&mut self) -> usize {
        loop {
            if !self.tick() {
                break self.count;
            }
        }
    }
}

pub fn _solve(input: String) -> String {
    let mut robot = CodeZero::from(&mut input.lines());
    robot.start().to_string()
}

#[cfg(test)]
mod p14503_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3 3
1 1 0
1 1 1
1 0 1
1 1 1"
            .to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "11 10
7 4 0
1 1 1 1 1 1 1 1 1 1
1 0 0 0 0 0 0 0 0 1
1 0 0 0 1 1 1 1 0 1
1 0 0 1 1 0 0 0 0 1
1 0 1 1 0 0 0 0 0 1
1 0 0 0 0 0 0 0 0 1
1 0 0 0 0 0 0 1 0 1
1 0 0 0 0 0 1 1 0 1
1 0 0 0 0 0 1 1 0 1
1 0 0 0 0 0 0 0 0 1
1 1 1 1 1 1 1 1 1 1"
            .to_string();
        let output = "57".to_string();
        assert_eq!(_solve(input), output);
    }
}
