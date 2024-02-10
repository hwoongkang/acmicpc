trait Unionable: Copy + Clone {
    type Delta;
    fn union(&self, other: &Self) -> Self;
    fn default() -> Self;
    fn apply(&self, delta: &Self::Delta) -> Self;
    fn delta(&self, other: &Self) -> Self::Delta;
}
impl Unionable for i64 {
    type Delta = (i64, i64);
    fn union(&self, other: &i64) -> i64 {
        self + other
    }
    fn default() -> i64 {
        0
    }
    fn apply(&self, delta: &(i64, i64)) -> i64 {
        self + delta.0 - delta.1
    }
    fn delta(&self, rhs: &Self) -> (i64, i64) {
        (*rhs, *self)
    }
}

struct FenwickTree<T>
where
    T: Unionable,
{
    size: usize,
    nums: Vec<T>,
    sums: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Unionable,
{
    fn sum(&self, mut i: usize) -> T {
        let mut ans = T::default();

        while i > 0 {
            ans = ans.union(&self.sums[i]);
            i -= 1 << i.trailing_zeros();
        }
        ans
    }

    fn update(&mut self, mut i: usize, value: T) {
        let delta = self.nums[i].delta(&value);
        self.nums[i] = value;
        while i <= self.size {
            self.sums[i] = self.sums[i].apply(&delta);
            i += 1 << i.trailing_zeros();
        }
    }

    fn new(size: usize) -> Self {
        Self {
            size,
            nums: vec![T::default(); size + 1],
            sums: vec![T::default(); size + 1],
        }
    }

    fn from(data: &[T]) -> Self {
        let size = data.len();

        let mut tree = Self::new(size);

        for (i, n) in data.iter().enumerate() {
            let i = i + 1;
            tree.update(i, *n);
        }

        tree
    }
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let n: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let nums: Vec<i64> = lines.take(n).map(|w| w.parse().unwrap()).collect();

    let mut tree: FenwickTree<i64> = FenwickTree::from(&nums);

    let mut ans: Vec<String> = vec![];

    for line in lines {
        let mut words = line.split_ascii_whitespace();
        if words.next().unwrap() == "1" {
            let i: usize = words.next().unwrap().parse().unwrap();
            let n: i64 = words.next().unwrap().parse().unwrap();
            tree.update(i, n);
        } else {
            let a: usize = words.next().unwrap().parse().unwrap();
            let b: usize = words.next().unwrap().parse().unwrap();
            ans.push((tree.sum(b) - tree.sum(a - 1)).to_string())
        }
    }

    ans.join("\n")
}

#[cfg(test)]
mod p02042_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5 2 2
1
2
3
4
5
1 3 6
2 2 5
1 5 2
2 3 5"
            .to_string();
        let output = "17
12"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
