fn _can_see(rev: &[usize], target: usize) -> usize {
    if rev.len() == 0 {
        return 0;
    }
    let mut start = 0;
    let mut end = rev.len() - 1;
    while start < end {
        let mid = (start + end) / 2;
        let num = rev[mid];
        if num > target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    let left_most = rev[start];

    // 4, 1
    if left_most > target {
        rev.len() - start
    } else {
        if start == 0 {
            rev.len()
        } else {
            rev.len() - start + 1
        }
    }
}
pub fn _solve(input: String) -> String {
    let people: Vec<usize> = input.lines().skip(1).map(|w| w.parse().unwrap()).collect();
    let mut stack: Vec<usize> = vec![];
    let mut ans = 0usize;
    for person in people {
        let now = _can_see(&stack, person);
        loop {
            if let Some(peek) = stack.pop() {
                if peek < person {
                    continue;
                } else {
                    stack.push(peek);
                    stack.push(person);
                    break;
                }
            } else {
                stack.push(person);
                break;
            }
        }
        ans += now;
    }
    ans.to_string()
}

#[cfg(test)]
mod p03015_tests {
    use super::*;

    #[test]
    fn test_can_see_1() {
        let rev = [5, 3, 2];
        let target = 3;
        assert_eq!(_can_see(&rev, target), 3);
    }
    #[test]
    fn test_can_see_2() {
        let rev = [5, 4, 2, 1];
        let target = 3;
        assert_eq!(_can_see(&rev, target), 3);
    }

    #[test]
    fn test_can_see_3() {
        let rev = [6, 5, 5, 2];
        let target = 5;
        assert_eq!(_can_see(&rev, target), 4);
    }
    #[test]
    fn test_can_see_4() {
        let rev = [8, 6];
        let target = 5;
        assert_eq!(_can_see(&rev, target), 1);
    }
    #[test]
    fn test_can_see_5() {
        let rev = [8, 7, 7];
        let target = 6;
        assert_eq!(_can_see(&rev, target), 1);
    }
    #[test]
    fn test_can_see_0() {
        let rev = [];
        let target = 5;
        assert_eq!(_can_see(&rev, target), 0);
    }

    #[test]
    fn test_case_1() {
        let input = "7
2
4
1
2
2
5
1"
        .to_string();
        let output = "10".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_2() {
        let input = "14
7
7
8
6
5
3
7
4
7
7
10
6
1
2"
        .to_string();
        let output = "25".to_string();
        assert_eq!(_solve(input), output);
    }
}
