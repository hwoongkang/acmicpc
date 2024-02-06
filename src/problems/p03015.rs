fn _search(nums: &[usize], e: usize, target: usize) -> (usize, usize) {
    if e == 0 {
        return (0, 0);
    }
    let mut start = 0;
    let mut end = e - 1;
    while start < end {
        let mid = (start + end) / 2;
        let num = nums[mid];
        if num > target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    let left_most = start;

    let mut start = 0;
    let mut end = e - 1;
    while start < end {
        let mid = (start + end) / 2;
        let num = nums[mid];
        if num < target {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    let right_most = end;

    let left_num = nums[left_most];

    let l = if left_num > target {
        // 이 경우가 나왔다는 것은: 모든 사람이 나보다 큰 경우기 때문에 1명만 보인다고 생각해도 됨.
        1
    } else if left_most == 0 {
        // 맨 앞까지 다 보이고 있는 경우
        e
    } else {
        // 여기까지가 나와 키가 같거나 작은 사람이므로, 한 명이 더 보임
        e - left_most + 1
    };

    let num = nums[right_most];
    let r = if num >= target {
        right_most + 1
    } else {
        right_most
    };
    (l, r)
}
pub fn _solve(input: String) -> String {
    let people: Vec<usize> = input.lines().skip(1).map(|w| w.parse().unwrap()).collect();
    let mut stack: Vec<usize> = vec![0; people.len()];
    let mut ans = 0usize;
    let mut len = 0;
    for person in people {
        let (add, index) = _search(&stack, len, person);
        stack[index] = person;
        len = index + 1;
        ans += add;
    }
    ans.to_string()
}

#[cfg(test)]
mod p03015_tests {
    use super::*;

    #[test]
    fn test_search_1() {
        let rev = [5, 3, 2];
        let target = 3;
        assert_eq!(_search(&rev, 3, target), (3, 2));
    }
    #[test]
    fn test_search_2() {
        let rev = [5, 4, 2, 1];
        let target = 3;
        assert_eq!(_search(&rev, 4, target), (3, 2));
    }

    #[test]
    fn test_search_3() {
        let rev = [6, 5, 5, 2];
        let target = 5;
        assert_eq!(_search(&rev, 4, target), (4, 3));
    }
    #[test]
    fn test_search_4() {
        let rev = [8, 6];
        let target = 5;
        assert_eq!(_search(&rev, 2, target), (1, 2));
    }
    #[test]
    fn test_search_5() {
        let rev = [8, 7, 7];
        let target = 6;
        assert_eq!(_search(&rev, 3, target), (1, 3));
    }
    #[test]
    fn test_search_0() {
        let rev = [];
        let target = 5;
        assert_eq!(_search(&rev, 0, target), (0, 0));
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
