#[allow(clippy::needless_collect)]
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let ants: Vec<usize> = lines.take(n).map(|line| line.parse().unwrap()).collect();
    let mut connections: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for line in lines {
        let mut words = line.split_ascii_whitespace();
        let a: usize = words.next().unwrap().parse().unwrap();
        let b: usize = words.next().unwrap().parse().unwrap();

        let a = a - 1;
        let b = b - 1;

        let cost: usize = words.next().unwrap().parse().unwrap();

        connections[a].push((b, cost));
        connections[b].push((a, cost));
    }

    let mut costs: Vec<Vec<(usize, usize)>> = vec![vec![]; n];

    costs[0] = vec![(0, 0)];

    let mut stack = vec![0];

    let mut visited = vec![false; n];
    visited[0] = true;

    while let Some(node) = stack.pop() {
        for &(child, cost) in connections[node].iter() {
            if visited[child] {
                continue;
            }
            visited[child] = true;
            let mut new_cost: Vec<_> = costs[node]
                .iter()
                .map(|&(ancestor, acc_cost)| (ancestor, acc_cost + cost))
                .collect();
            new_cost.push((child, 0));
            costs[child] = new_cost;
            stack.push(child);
        }
    }

    ants.into_iter()
        .enumerate()
        .map(|(ant, cap)| {
            for &(node, cost) in costs[ant].iter() {
                if cost <= cap {
                    return node + 1;
                }
            }
            ant + 1
        })
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod p14942_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "4
10
8
22
18
1 2 10
2 3 10
2 4 10"
            .to_string();
        let output = "1
2
1
2"
        .to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_2() {
        let input = "9
49
17
45
27
49
33
34
14
28
2 3 6
5 9 10
4 1 8
5 1 6
9 8 3
8 3 1
2 6 10
4 7 10"
            .to_string();
        let output = "1
9
1
1
1
5
1
5
1"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
