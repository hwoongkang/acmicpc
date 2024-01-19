pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut nums: Vec<usize> = lines
        .skip(1)
        .next()
        .unwrap()
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    nums.sort();

    let targets: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    let l = nums.len();
    let mut offsets: Vec<usize> = (0..l).map(|_| 0).collect();

    let binary_search = |target: usize| -> usize {
        let mut s = 0;
        let mut e = l - 1;
        loop {
            if s >= e {
                break e;
            }
            let m = (s + e) / 2;
            if nums[m] > target {
                e = m;
            } else {
                s = m + 1
            }
        }
    };

    let mut answers = vec![];

    for target in targets.into_iter() {
        let i = binary_search(target);
        let mut j = i;
        loop {
            let offset = offsets[j];
            if offset == 0 {
                break;
            }
            j += offset;
        }

        let ans = nums[j];
        answers.push(ans.to_string());

        let new_offset = j - i + 1;
        for j in 0..new_offset {
            offsets[i + j] = new_offset - j;
        }
    }

    answers.join("\n")
}

// 1 2 3 4 5 6 7
// 4 5
// 4 6
// 2 3
// 2 7

#[cfg(test)]
mod p16566_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "10 7 5
2 5 3 7 8 4 9
4 1 1 3 8"
            .to_string();
        let output = "5
2
3
4
9"
        .to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_part_2() {
        let input = "10 7 5
1 2 3 4 5 6 7
4 4 3 3"
            .to_string();
        let output = "5
6
4
7"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
