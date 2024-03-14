use std::{cmp::Reverse, collections::BinaryHeap};

enum Tile {
    Safe,
    Danger,
    Lethal,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct State {
    cost: usize,
    pos: (usize, usize),
}

pub fn _solve(input: String) -> String {
    let mut map: Vec<Vec<Tile>> = (0..=500)
        .map(|_| (0..=500).map(|_| Tile::Safe).collect())
        .collect();
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    for line in lines.take(n) {
        let mut words = line.split_ascii_whitespace();
        let x1: usize = words.next().unwrap().parse().unwrap();
        let y1: usize = words.next().unwrap().parse().unwrap();
        let x2: usize = words.next().unwrap().parse().unwrap();
        let y2: usize = words.next().unwrap().parse().unwrap();
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));
        for row in map.iter_mut().take(x2 + 1).skip(x1) {
            for tile in row.iter_mut().take(y2 + 1).skip(y1) {
                *tile = Tile::Danger;
            }
        }
    }
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    for line in lines.take(n) {
        let mut words = line.split_ascii_whitespace();
        let x1: usize = words.next().unwrap().parse().unwrap();
        let y1: usize = words.next().unwrap().parse().unwrap();
        let x2: usize = words.next().unwrap().parse().unwrap();
        let y2: usize = words.next().unwrap().parse().unwrap();
        let (x1, x2) = (x1.min(x2), x1.max(x2));
        let (y1, y2) = (y1.min(y2), y1.max(y2));
        for row in map.iter_mut().take(x2 + 1).skip(x1) {
            for tile in row.iter_mut().take(y2 + 1).skip(y1) {
                *tile = Tile::Lethal;
            }
        }
    }

    let mut distances: Vec<Vec<usize>> = vec![vec![usize::MAX; 501]; 501];

    let mut heap = BinaryHeap::from([Reverse(State {
        cost: 0,
        pos: (0, 0),
    })]);

    distances[0][0] = 0;

    while let Some(Reverse(State { cost, pos })) = heap.pop() {
        if pos == (500, 500) {
            return cost.to_string();
        }

        if distances[pos.0][pos.1] < cost {
            continue;
        }

        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (r, c) = pos;
            let r = r as i32 + dr;
            let c = c as i32 + dc;
            if !(0..=500).contains(&r) || !(0..=500).contains(&c) {
                continue;
            }
            let r = r as usize;
            let c = c as usize;
            match map[r][c] {
                Tile::Lethal => {}
                Tile::Safe => {
                    if cost < distances[r][c] {
                        distances[r][c] = cost;
                        heap.push(Reverse(State { cost, pos: (r, c) }))
                    }
                }
                Tile::Danger => {
                    let cost = cost + 1;
                    if cost < distances[r][c] {
                        distances[r][c] = cost;
                        heap.push(Reverse(State { cost, pos: (r, c) }))
                    }
                }
            }
        }
    }

    "-1".to_string()
}

#[cfg(test)]
mod p01584_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "1
500 0 0 500
1
0 0 0 0"
            .to_string();
        let output = "1000".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "0
0"
        .to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "2
0 0 250 250
250 250 500 500
2
0 251 249 500
251 0 500 249"
            .to_string();
        let output = "1000".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "2
0 0 250 250
250 250 500 500
2
0 250 250 500
250 0 500 250"
            .to_string();
        let output = "-1".to_string();
        assert_eq!(_solve(input), output);
    }
}
