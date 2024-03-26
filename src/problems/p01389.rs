pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();

    let n: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();

    let mut dists: Vec<Vec<usize>> = vec![vec![1_000_000_000; n]; n];

    for line in lines {
        let mut words = line.split_ascii_whitespace();
        let i: usize = words.next().unwrap().parse().unwrap();
        let j: usize = words.next().unwrap().parse().unwrap();
        let i = i - 1;
        let j = j - 1;
        dists[i][j] = 1;
        dists[j][i] = 1;
    }
    for i in 0..n {
        dists[i][i] = 0;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let roundtrip = dists[i][k] + dists[k][j];
                dists[i][j] = roundtrip.min(dists[i][j]);
            }
        }
    }

    let ans = dists
        .into_iter()
        .map(|row| row.into_iter().sum::<usize>())
        .enumerate()
        .min_by_key(|(i, n)| (*n, *i))
        .unwrap()
        .0;
    let ans = ans + 1;

    ans.to_string()
}

#[cfg(test)]
mod p01389_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5 5
1 3
1 4
4 5
4 3
3 2"
        .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }
}
