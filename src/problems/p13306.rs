struct _DisjointSet {
    arr: Vec<(usize, usize)>,
}
impl _DisjointSet {
    fn _new(n: usize) -> Self {
        let arr = (0..n).map(|i| (i, 0)).collect();
        Self { arr }
    }
    fn _find(&mut self, a: usize) -> usize {
        if self.arr[a].0 != a {
            self.arr[a].0 = self._find(self.arr[a].0);
        }
        self.arr[a].0
    }
    fn _union(&mut self, a: usize, b: usize) {
        let a_root = self._find(a);
        let b_root = self._find(b);
        if a_root == b_root {
            return;
        }
        let a_rank = self.arr[a_root].1;
        let b_rank = self.arr[b_root].1;
        if a_rank < b_rank {
            self.arr[a_root].0 = b_root;
        } else if a_rank > b_rank {
            self.arr[b_root].0 = a_root;
        } else {
            self.arr[b_root].0 = a_root;
            self.arr[a_root].1 += 1;
        }
    }
    fn _query(&mut self, a: usize, b: usize) -> bool {
        self._find(a) == self._find(b)
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
    let mut parents = vec![0usize; n];
    for (child, line) in lines.take(n - 1).enumerate() {
        let child = child + 1;
        let parent: usize = line.parse().unwrap();
        let parent = parent - 1;
        parents[child] = parent;
    }

    let mut set = _DisjointSet::_new(n);

    let mut ans = Vec::new();

    for line in lines.rev() {
        let cmd: Vec<usize> = line
            .split_ascii_whitespace()
            .map(|w| w.parse().unwrap())
            .collect();
        if cmd[0] == 0 {
            let node = cmd[1] - 1;
            set._union(node, parents[node]);
        } else {
            let a = cmd[1] - 1;
            let b = cmd[2] - 1;
            if set._query(a, b) {
                ans.push("YES");
            } else {
                ans.push("NO");
            }
        }
    }

    ans.into_iter().rev().collect::<Vec<_>>().join("\n")
}

#[cfg(test)]
mod p13306_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3 3
1
1
1 2 3
0 3
1 2 3
1 1 2
0 2"
        .to_string();
        let output = "YES
NO
YES"
        .to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_2() {
        let input = "11 5
7
4
1
9
11
1
11
1
3
7
0 11
1 8 5
1 3 9
0 10
0 9
0 7
1 2 7
0 5
1 1 10
0 8
0 6
0 2
1 1 3
0 3
0 4"
        .to_string();
        let output = "NO
YES
YES
NO
YES"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
