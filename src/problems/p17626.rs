use std::collections::VecDeque;

pub fn _solve(input: String) -> String {
    let mut visited = [false; 50_001];

    let n: usize = input.lines().next().unwrap().parse().unwrap();

    visited[n] = true;

    let mut queue = VecDeque::from([(n, 0)]);

    while let Some((n, cost)) = queue.pop_front() {
        if n == 0 {
            return cost.to_string();
        }
        let cost = cost + 1;
        for m in 1.. {
            let sq = m * m;
            if sq > n {
                break;
            }
            let new = n - sq;
            if visited[new] {
                continue;
            }
            visited[new] = true;
            queue.push_back((new, cost));
        }
    }

    String::new()
}

#[cfg(test)]
mod p17626_test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "25".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_2() {
        let input = "26".to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_3() {
        let input = "20".to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_4() {
        let input = "19".to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_5() {
        let input = "11339".to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_6() {
        let input = "34567".to_string();
        let output = "4".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_7() {
        let input = "1".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_8() {
        let input = "50000".to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }
}
