trait Unionable: Copy + Clone {
    fn union(&self, other: &Self) -> Self;
    fn default() -> Self;
}

impl Unionable for i64 {
    fn union(&self, other: &i64) -> i64 {
        (self * other) % 1_000_000_007
    }
    fn default() -> i64 {
        1
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
    let n: usize = lines
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let nums: Vec<i64> = lines.take(n).map(|w| w.parse().unwrap()).collect();

    let mut tree: SegmentTree<i64> = SegmentTree::from(&nums);

    let mut ans: Vec<String> = vec![];

    for line in lines {
        let mut words = line.split_ascii_whitespace();
        if words.next().unwrap() == "1" {
            let i: usize = words.next().unwrap().parse().unwrap();
            let n: i64 = words.next().unwrap().parse().unwrap();
            tree.update(i - 1, n);
        } else {
            let a: usize = words.next().unwrap().parse().unwrap();
            let b: usize = words.next().unwrap().parse().unwrap();
            ans.push(tree.query(a - 1, b).to_string())
        }
    }

    ans.join("\n")
}

#[cfg(test)]
mod p11505_tests {
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
        let output = "240
48"
        .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "5 2 2
1
2
3
4
5
1 3 0
2 2 5
1 3 6
2 2 5"
            .to_string();
        let output = "0
240"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
