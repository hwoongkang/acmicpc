pub fn _solve(input: String) -> String {
    let castle: Vec<Vec<char>> = input
        .lines()
        .skip(1)
        .map(|line| line.chars().collect())
        .collect();
    let mr = castle.len();
    let mc = castle[0].len();

    let rows: usize = (0..mr)
        .map(|r| {
            (0..mc)
                .map(|c| usize::from(castle[r][c] == '.'))
                .sum::<usize>()
        })
        .filter_map(|n| if n == 0 { Some(()) } else { None })
        .count();
    let cols: usize = (0..mc)
        .map(|c| {
            (0..mr)
                .map(|r| usize::from(castle[r][c] == '.'))
                .sum::<usize>()
        })
        .filter_map(|n| if n == 0 { Some(()) } else { None })
        .count();

    rows.max(cols).to_string()
}

#[cfg(test)]
mod p01236_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "4 4
....
....
....
...."
            .to_string();
        let output = "4".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "3 5
XX...
.XX..
...XX"
            .to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "5 8
....XXXX
........
XX.X.XX.
........
........"
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }
}
