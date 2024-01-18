use std::collections::VecDeque;
pub fn _solve(input: String) -> String {
    let mut portals: Vec<usize> = (0..=100).map(|n| n).collect();
    for line in input.lines().skip(1) {
        let mut words = line.split_whitespace();
        let from: usize = words.next().unwrap().parse().unwrap();
        let to: usize = words.next().unwrap().parse().unwrap();
        portals[from] = to;
    }
    let mut queue = VecDeque::from([(1, 0)]);
    let mut visited: Vec<bool> = (0..=100).map(|_| false).collect();

    visited[1] = true;

    while let Some((n, cost)) = queue.pop_front() {
        let cost = cost + 1;
        for i in 1..=6usize {
            let m = n + i;
            if m == 100 {
                return cost.to_string();
            }
            let m = portals[m];
            if !visited[m] {
                visited[m] = true;
                queue.push_back((m, cost));
            }
        }
    }

    String::new()
}

#[cfg(test)]
mod p16928_test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "3 7
32 62
42 68
12 98
95 13
97 25
93 37
79 27
75 19
49 47
67 17"
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_2() {
        let input = "4 9
8 52
6 80
26 42
2 72
51 19
39 11
37 29
81 3
59 5
79 23
53 7
43 33
77 21"
            .to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }
}
