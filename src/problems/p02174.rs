type Pos = (usize, usize);

enum Dir {
    North,
    West,
    South,
    East,
}

use Dir::*;

impl Dir {
    fn from(s: &str) -> Self {
        match s {
            "N" => North,
            "S" => South,
            "W" => West,
            "E" => East,
            _ => unreachable!(),
        }
    }

    fn left(&self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    fn right(&self) -> Self {
        self.left().left().left()
    }

    fn forward(&self, pos: &Pos, clamp: &Pos) -> Option<Pos> {
        let r = pos.0 as i32;
        let c = pos.1 as i32;
        let mr = clamp.0 as i32;
        let mc = clamp.1 as i32;
        let (r, c) = match self {
            North => (r + 1, c),
            South => (r - 1, c),
            West => (r, c - 1),
            East => (r, c + 1),
        };
        if r < 0 || r >= mr || c < 0 || c >= mc {
            None
        } else {
            Some((r as usize, c as usize))
        }
    }
}
struct Robot {
    pos: Pos,
    dir: Dir,
}

impl Robot {
    fn from(s: &str) -> Self {
        let mut words = s.split_ascii_whitespace();
        let c: usize = words.next().unwrap().parse().unwrap();
        let r: usize = words.next().unwrap().parse().unwrap();
        let pos = (r - 1, c - 1);
        let dir = Dir::from(words.next().unwrap());
        Self { pos, dir }
    }
}

struct Map {
    robots: Vec<Robot>,
    tiles: Vec<Vec<Option<usize>>>,
}

enum Command {
    TurnLeft,
    TurnRight,
    Move,
}

use Command::*;

impl Command {
    fn from(s: &str) -> Self {
        match s {
            "L" => TurnLeft,
            "R" => TurnRight,
            "F" => Move,
            _ => unreachable!(),
        }
    }
}

enum RobotErr {
    Wall(usize),
    Crash(usize, usize),
}

impl std::fmt::Display for RobotErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall(i) => {
                write!(f, "Robot {} crashes into the wall", i + 1)
            }
            Self::Crash(i, j) => {
                write!(f, "Robot {} crashes into robot {}", i + 1, j + 1)
            }
        }
    }
}

impl Map {
    fn cmd(&mut self, index: usize, cmd: &Command) -> Result<(), RobotErr> {
        match cmd {
            TurnLeft => {
                self.robots[index].dir = self.robots[index].dir.left();
                Ok(())
            }
            TurnRight => {
                self.robots[index].dir = self.robots[index].dir.right();
                Ok(())
            }
            Move => {
                let clamp = self.size();
                let Robot { dir, pos } = &mut self.robots[index];
                let Some((r, c)) = dir.forward(pos, &clamp) else {
                    return Err(RobotErr::Wall(index))
                };
                if let Some(j) = self.tiles[r][c] {
                    return Err(RobotErr::Crash(index, j));
                }
                self.tiles[pos.0][pos.1] = None;
                self.tiles[r][c] = Some(index);
                *pos = (r, c);
                Ok(())
            }
        }
    }
    fn size(&self) -> Pos {
        (self.tiles.len(), self.tiles[0].len())
    }

    fn from(lines: &mut dyn Iterator<Item = &str>) -> Self {
        let line = lines.next().unwrap();
        let mut words = line.split_ascii_whitespace();
        let mc: usize = words.next().unwrap().parse().unwrap();
        let mr: usize = words.next().unwrap().parse().unwrap();
        let mut tiles = vec![vec![None; mc]; mr];
        let mut robots = vec![];
        let line = lines.next().unwrap();
        let n: usize = line
            .split_ascii_whitespace()
            .next()
            .unwrap()
            .parse()
            .unwrap();
        for (i, line) in lines.take(n).enumerate() {
            let robot = Robot::from(line);
            let (r, c) = robot.pos;
            tiles[r][c] = Some(i);
            robots.push(robot);
        }
        Self { tiles, robots }
    }
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut map = Map::from(lines);
    for line in lines {
        let mut words = line.split_ascii_whitespace();
        let i: usize = words.next().unwrap().parse().unwrap();
        let i = i - 1;
        let cmd = Command::from(words.next().unwrap());
        let it: usize = words.next().unwrap().parse().unwrap();

        for _ in 0..it {
            if let Err(err) = map.cmd(i, &cmd) {
                return format!("{}", err);
            }
        }
    }
    "OK".to_string()
}

#[cfg(test)]
mod p02174_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5 4
2 2
1 1 E
5 4 W
1 F 7
2 F 7"
            .to_string();
        let output = "Robot 1 crashes into the wall".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "1 1
1 1
1 1 E
1 F 1"
            .to_string();
        let output = "Robot 1 crashes into the wall".to_string();
        assert_eq!(_solve(input), output);
    }
}
