pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();

    let mut min_max = [(0, 0); 3];

    let line = lines.nth(1).unwrap();

    let points: Vec<usize> = line
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    for i in 0..3 {
        let n = points[i];
        min_max[i] = (n, n);
    }

    for line in lines {
        let a = min_max[0];
        let b = min_max[1];
        let c = min_max[2];

        let mut words = line.split_ascii_whitespace();

        let d: usize = words.next().unwrap().parse().unwrap();
        let e: usize = words.next().unwrap().parse().unwrap();
        let f: usize = words.next().unwrap().parse().unwrap();

        let d0 = (a.0).min(b.0) + d;
        let d1 = (a.1).max(b.1) + d;

        let e0 = (a.0).min(b.0).min(c.0) + e;
        let e1 = (a.1).max(b.1).max(c.1) + e;

        let f0 = (b.0).min(c.0) + f;
        let f1 = (b.1).max(c.1) + f;

        min_max = [(d0, d1), (e0, e1), (f0, f1)];
    }

    let min = min_max.iter().map(|n| n.0).min().unwrap();
    let max = min_max.iter().map(|n| n.1).max().unwrap();
    format!("{} {}", max, min)
}

#[cfg(test)]
mod p02096_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
1 2 3
4 5 6
4 9 0"
            .to_string();
        let output = "18 6".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "3
0 0 0
0 0 0
0 0 0"
            .to_string();
        let output = "0 0".to_string();
        assert_eq!(_solve(input), output);
    }
}
