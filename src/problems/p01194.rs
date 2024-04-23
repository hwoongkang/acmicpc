use std::collections::VecDeque;

/**
 * 빈 칸: 언제나 이동할 수 있다. ('.')
벽: 절대 이동할 수 없다. ('#')
열쇠: 언제나 이동할 수 있다. 이 곳에 처음 들어가면 열쇠를 집는다. ('a', 'b', 'c', 'd', 'e', 'f')
문: 대응하는 열쇠가 있을 때만 이동할 수 있다. ('A', 'B', 'C', 'D', 'E', 'F')
민식이의 현재 위치: 빈 곳이고, 민식이가 현재 서 있는 곳이다. ('0')
출구: 달이 차오르기 때문에, 민식이가 가야하는 곳이다. 이 곳에 오면 미로를 탈출한다. ('1')
 */
#[derive(Debug)]
enum Tile {
    Empty,
    Wall,
    Key(usize),
    Door(usize),
    Exit,
}
#[derive(Debug)]
struct State {
    keys: usize,
    pos: (usize, usize),
    dist: usize,
}

pub fn _solve(input: String) -> String {
    let mut start = (0, 0);
    let tiles: Vec<Vec<Tile>> = input
        .lines()
        .skip(1)
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '.' => Tile::Empty,
                    '0' => {
                        start = (r, c);
                        Tile::Empty
                    }
                    '#' => Tile::Wall,
                    '1' => Tile::Exit,
                    c => {
                        if c.is_ascii_lowercase() {
                            let c = c as usize - 'a' as usize;
                            Tile::Key(1 << c)
                        } else {
                            let c = c as usize - 'A' as usize;
                            Tile::Door(1 << c)
                        }
                    }
                })
                .collect()
        })
        .collect();

    let state = State {
        pos: start,
        dist: 0,
        keys: 0,
    };

    let mr = tiles.len();
    let mc = tiles[0].len();

    let mut visited = vec![vec![vec![false; mc]; mr]; 64];

    visited[0][start.0][start.1] = true;

    let mut queue = VecDeque::from([state]);

    while let Some(state) = queue.pop_front() {
        let State { pos, dist, keys } = state;
        let r = pos.0 as i32;
        let c = pos.1 as i32;
        let dist = dist + 1;
        for (dr, dc) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let r = r + dr;
            let c = c + dc;
            if r < 0 || c < 0 {
                continue;
            }
            let r = r as usize;
            let c = c as usize;
            if r >= mr || c >= mc {
                continue;
            }
            match tiles[r][c] {
                Tile::Exit => return dist.to_string(),
                Tile::Wall => {}
                Tile::Door(n) => {
                    if (keys & n > 0) && !visited[keys][r][c] {
                        visited[keys][r][c] = true;
                        queue.push_back(State {
                            pos: (r, c),
                            dist,
                            keys,
                        });
                    }
                }
                Tile::Empty => {
                    if !visited[keys][r][c] {
                        visited[keys][r][c] = true;
                        queue.push_back(State {
                            pos: (r, c),
                            dist,
                            keys,
                        })
                    }
                }
                Tile::Key(n) => {
                    let keys = keys | n;
                    if !visited[keys][r][c] {
                        visited[keys][r][c] = true;
                        queue.push_back(State {
                            pos: (r, c),
                            dist,
                            keys,
                        })
                    }
                }
            }
        }
    }

    "-1".to_string()
}

#[cfg(test)]
mod p01194_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "1 7
f0.F..1"
            .to_string();
        let output = "7".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "5 5
....1
#1###
.1.#0
....A
.1.#."
            .to_string();
        let output = "-1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "7 8
a#c#eF.1
.#.#.#..
.#B#D###
0....F.1
C#E#A###
.#.#.#..
d#f#bF.1"
            .to_string();
        let output = "55".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "3 4
1..0
###.
1..."
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_5() {
        let input = "3 5
..0..
.###.
..1.A"
            .to_string();
        let output = "6".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_6() {
        let input = "4 5
0....
.#B#A
.#.#.
b#a#1"
            .to_string();
        let output = "19".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_7() {
        let input = "1 11
c.0.C.C.C.1"
            .to_string();
        let output = "12".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_8() {
        let input = "3 6
###...
#0A.1a
###..."
            .to_string();
        let output = "-1".to_string();
        assert_eq!(_solve(input), output);
    }
}
