trait Unionable: Copy + Clone {
    fn union(&self, other: &Self) -> Self;
    fn default() -> Self;
}

impl Unionable for i64 {
    fn union(&self, other: &i64) -> i64 {
        self + other
    }
    fn default() -> i64 {
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
    let nums: Vec<i64> = lines
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    let mut tree = SegmentTree::from(&nums);

    let mut ans: Vec<String> = vec![];
    for line in lines {
        let mut words = line.split_ascii_whitespace();
        let x: usize = words.next().unwrap().parse().unwrap();
        let y: usize = words.next().unwrap().parse().unwrap();
        let a: usize = words.next().unwrap().parse().unwrap();
        let b: i64 = words.next().unwrap().parse().unwrap();
        let (x, y) = if x < y { (x, y) } else { (y, x) };
        ans.push(tree.query(x - 1, y).to_string());
        tree.update(a - 1, b);
    }
    ans.join("\n")
}

#[cfg(test)]
mod p01275_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5 2
1 2 3 4 5
2 3 3 1
5 3 4 1"
            .to_string();
        let output = "5
10"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
