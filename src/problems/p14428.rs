#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct State(usize, usize);

trait Unionable: Copy + Clone {
    fn union(&self, other: &Self) -> Self;
    fn default() -> Self;
}

impl Unionable for State {
    fn union(&self, other: &State) -> State {
        *self.min(other)
    }
    fn default() -> State {
        State(usize::MAX, usize::MAX)
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
    let data: Vec<State> = lines
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|w| w.parse().unwrap())
        .enumerate()
        .map(|(i, num)| State(num, i))
        .collect();
    let mut tree = SegmentTree::from(&data);

    let mut ans = vec![];
    for line in lines.skip(1) {
        let cmd: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|w| w.parse().unwrap())
            .collect();

        if cmd[0] == 1 {
            let index = cmd[1] - 1;
            let value = cmd[2];
            tree.update(index, State(value, index));
        } else {
            let from = cmd[1] - 1;
            let to = cmd[2];
            let state = tree.query(from, to);
            let index = state.1 + 1;
            ans.push(index.to_string());
        }
    }
    ans.join("\n")
}

#[cfg(test)]
mod p14428_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5
5 4 3 2 1
6
2 1 3
2 1 4
1 5 3
2 3 5
1 4 3
2 3 5"
            .to_string();
        let output = "3
4
4
3"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
