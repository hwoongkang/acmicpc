pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut from_left = vec![1; n];
        for (i, j) in (0..n).zip(1..n) {
            if ratings[j] > ratings[i] {
                from_left[j] = from_left[i] + 1;
            }
        }
        let mut from_right = vec![1; n];
        for (i, j) in (0..n).rev().zip((0..n - 1).rev()) {
            if ratings[j] > ratings[i] {
                from_right[j] = from_right[i] + 1;
            }
        }

        from_left
            .into_iter()
            .zip(from_right.into_iter())
            .map(|(i, j)| i.max(j))
            .sum::<i32>()
    }
}

#[cfg(test)]
mod leet_00135_tests {
    use super::*;

    #[test]
    fn test_01() {
        let input = vec![1, 0, 2];
        let ans = 5;
        assert_eq!(ans, Solution::candy(input));
    }
    #[test]
    fn test_02() {
        let input = vec![1, 2, 2];
        let ans = 4;
        assert_eq!(ans, Solution::candy(input));
    }
    #[test]
    fn test_03() {
        let input = vec![1, 2, 3, 0, 3, 2, 1];
        let ans = 13;
        assert_eq!(ans, Solution::candy(input));
    }
    #[test]
    fn test_04() {
        let input = vec![1, 2, 3, 0, 3, 2, 3];
        let ans = 12;
        assert_eq!(ans, Solution::candy(input));
    }
}
