use std::collections::HashMap;

trait Unionable: Copy + Clone {
    fn union(&self, other: &Self) -> Self;
    fn default() -> Self;
}

impl Unionable for u64 {
    fn union(&self, other: &u64) -> u64 {
        self + other
    }
    fn default() -> u64 {
        0
    }
}
struct SegmentTree<T>
where
    T: Unionable,
{
    data: Vec<T>,
}

impl<T> SegmentTree<T>
where
    T: Unionable,
{
    fn from(arr: &[T]) -> Self {
        let n = arr.len();
        let mut data = vec![T::default(); n * 2];
        for (i, val) in arr.iter().enumerate() {
            data[n + i] = *val;
        }
        for i in (1..n).rev() {
            data[i] = data[i << 1].union(&data[i << 1 | 1]);
        }
        Self { data }
    }

    fn size(&self) -> usize {
        self.data.len() / 2
    }

    fn update(&mut self, i: usize, val: T) {
        let n = self.size();
        let mut i = n + i;
        self.data[i] = val;
        while i > 1 {
            self.data[i >> 1] = self.data[i].union(&self.data[i ^ 1]);
            i >>= 1;
        }
    }

    fn query(&self, l: usize, r: usize) -> T {
        let mut ans = T::default();
        let n = self.size();
        let mut l = l + n;
        let mut r = r + n;
        while l < r {
            if l & 1 != 0 {
                ans = ans.union(&self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                ans = ans.union(&self.data[r]);
            }
            l >>= 1;
            r >>= 1;
        }
        ans
    }
}
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let map: HashMap<usize, usize> = lines
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| -> usize { w.parse().unwrap() })
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect();
    let len = map.len();
    let mut tree = SegmentTree::from(&vec![0; len]);
    let mut ans = 0;
    for word in lines.next().unwrap().split_ascii_whitespace() {
        let num: usize = word.parse().unwrap();
        let index = *map.get(&num).unwrap();
        ans += tree.query(index, len);
        tree.update(index, 1);
    }
    ans.to_string()
}

#[cfg(test)]
mod p07578_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5
132 392 311 351 231
392 351 132 311 231"
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }
}
