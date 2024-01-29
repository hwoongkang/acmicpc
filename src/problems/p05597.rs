pub fn _solve(input: String) -> String {
    let mut submitted = [false; 30];
    for line in input.lines() {
        let n: usize = line.parse().unwrap();
        let n = n - 1;
        submitted[n] = true;
    }
    submitted
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b { None } else { Some(i + 1) })
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod p05597_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
1
4
5
7
9
6
10
11
12
13
14
15
16
17
18
19
20
21
22
23
24
25
26
27
28
29
30"
        .to_string();
        let output = "2
8"
        .to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_2() {
        let input = "9
30
6
12
10
20
21
11
7
5
28
4
18
29
17
19
27
13
16
26
14
23
22
15
3
1
24
25"
        .to_string();
        let output = "2
8"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
