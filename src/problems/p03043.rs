fn parse_input(input: String) -> (usize, Vec<(usize, usize)>) {
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let tanks: Vec<(usize, usize)> = lines
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            let x: usize = words.next().unwrap().parse().unwrap();
            let y: usize = words.next().unwrap().parse().unwrap();
            (x - 1, y - 1)
        })
        .collect();
    (n, tanks)
}

pub fn _solve(input: String) -> String {
    let (n, tanks) = parse_input(input);
    let mut x_indices: Vec<(usize, usize)> = tanks
        .iter()
        .enumerate()
        .map(|(i, &(x, _))| (x, i))
        .collect();
    let mut y_indices: Vec<(usize, usize)> = tanks
        .iter()
        .enumerate()
        .map(|(i, &(_, y))| (y, i))
        .collect();

    x_indices.sort_by_key(|t| t.0);
    y_indices.sort_by_key(|t| t.0);
    let mut ans: Vec<String> = vec![];

    for i in 0..n {
        let (x, j) = x_indices[i];
        if x > i {
            let amount = x - i;
            for _ in 0..amount {
                ans.push(format!("{} U", j + 1))
            }
        }
    }
    for i in (0..n).rev() {
        let (x, j) = x_indices[i];
        if x < i {
            let amount = i - x;
            for _ in 0..amount {
                ans.push(format!("{} D", j + 1))
            }
        }
    }

    for i in 0..n {
        let (y, j) = y_indices[i];
        if y > i {
            let amount = y - i;
            for _ in 0..amount {
                ans.push(format!("{} L", j + 1))
            }
        }
    }

    for i in (0..n).rev() {
        let (y, j) = y_indices[i];
        if y < i {
            let amount = i - y;
            for _ in 0..amount {
                ans.push(format!("{} R", j + 1))
            }
        }
    }

    let mut str = ans.len().to_string();
    for s in ans {
        str += "\n";
        str += &s;
    }
    str
}

fn special_judge(input: String, output: String) -> bool {
    let (n, mut tanks) = parse_input(input);
    let mut board = vec![vec![false; n]; n];
    for &(i, j) in tanks.iter() {
        board[i][j] = true;
    }
    for line in output.lines().skip(1) {
        let mut words = line.split_ascii_whitespace();
        let i: usize = words.next().unwrap().parse().unwrap();
        let i = i - 1;
        let (x, y) = &mut tanks[i];
        board[*x][*y] = false;
        match words.next().unwrap() {
            "U" => *x -= 1,
            "D" => *x += 1,
            "L" => *y -= 1,
            "R" => *y += 1,
            _ => unreachable!(),
        }
        if board[*x][*y] {
            return false;
        }
        board[*x][*y] = true;
    }
    let mut visited = vec![false; n];
    for &(x, _) in tanks.iter() {
        visited[x] = true;
    }
    if visited
        .into_iter()
        .filter_map(|b| if !b { Some(()) } else { None })
        .count()
        != 0
    {
        return false;
    }
    let mut visited = vec![false; n];
    for &(_, y) in tanks.iter() {
        visited[y] = true;
    }
    if visited
        .into_iter()
        .filter_map(|b| if !b { Some(()) } else { None })
        .count()
        != 0
    {
        return false;
    }
    true
}

#[cfg(test)]
mod p03043_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5
1 1
1 2
1 3
1 4
1 5"
        .to_string();
        let output = _solve(input.clone());
        println!("{:?}", output);
        assert!(special_judge(input, output));
    }

    #[test]
    fn test_case_2() {
        let input = "5
2 3
3 2
3 3
3 4
4 3"
        .to_string();
        let output = _solve(input.clone());
        println!("{:?}", output);
        assert!(special_judge(input, output));
    }

    #[test]
    fn test_case_3() {
        let input = "6
1 1
1 2
2 1
5 6
6 5
6 6"
        .to_string();

        let output = _solve(input.clone());
        println!("{:?}", output);
        assert!(special_judge(input, output));
    }

    #[test]
    fn test_case_4() {
        let input = "5
1 1
1 2
2 2
2 3
2 4"
        .to_string();
        let output = _solve(input.clone());
        println!("{:?}", output);
        assert!(special_judge(input, output));
    }
}
