trait Unionable: Copy + Clone {
    fn union(&self, other: &Self) -> Self;
    fn default() -> Self;
}

impl Unionable for u32 {
    fn union(&self, other: &u32) -> u32 {
        self + other
    }
    fn default() -> u32 {
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

fn test_case(n: usize, m: usize, queries: Vec<usize>) -> String {
    let mut top = n - 1;
    let mut currently_on: Vec<usize> = (0..n).rev().collect();
    let data: Vec<u32> = (0..n).map(|_| 1).chain((0..m).map(|_| 0)).collect();
    let mut tree = SegmentTree::from(&data);

    let mut ans: Vec<String> = vec![];

    for i in queries.into_iter() {
        let prev = currently_on[i];
        tree.update(prev, 0);

        ans.push(tree.query(prev, n + m).to_string());
        top += 1;
        currently_on[i] = top;
        tree.update(top, 1);
    }

    ans.join(" ")
}
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let testcases: usize = lines.next().unwrap().parse().unwrap();
    (0..testcases)
        .map(|_| {
            let mut first_line = lines.next().unwrap().split_ascii_whitespace();
            let n: usize = first_line.next().unwrap().parse().unwrap();
            let m: usize = first_line.next().unwrap().parse().unwrap();
            let nums: Vec<usize> = lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .map(|n: usize| n - 1)
                .collect();
            test_case(n, m, nums)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod p03653_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "2
3 3
3 1 1
5 3
4 4 5"
            .to_string();
        let output = "2 1 0
3 0 4"
            .to_string();
        assert_eq!(_solve(input), output);
    }
}
