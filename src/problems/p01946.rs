pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let testcases: usize = lines.next().unwrap().parse().unwrap();

    let mut ans = vec![];

    for _ in 0..testcases {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let mut ranks: Vec<(usize, usize)> = lines
            .take(n)
            .map(|line| {
                let mut line = line.split_ascii_whitespace();
                (
                    line.next().unwrap().parse().unwrap(),
                    line.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        ranks.sort_by_key(|n| n.0);

        let mut count = 0;
        let mut best = usize::MAX;
        for rank in ranks.iter().map(|p| p.1) {
            if rank <= best {
                count += 1;
                best = rank;
            }
        }
        ans.push(count.to_string());
    }

    ans.join("\n")
}

#[cfg(test)]
mod p01946_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "2
5
3 2
1 4
4 1
2 3
5 5
7
3 6
7 3
4 2
1 4
5 7
2 5
6 1"
        .to_string();
        let output = "4
3"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
