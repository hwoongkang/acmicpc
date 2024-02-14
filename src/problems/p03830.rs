#[derive(Copy, Clone)]
struct State {
    parent: usize,
    rank: usize,
    dist: i64, // to parent
}

struct DisjointSet {
    arr: Vec<State>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        Self {
            arr: (0..n)
                .map(|i| State {
                    parent: i,
                    rank: 0,
                    dist: 0,
                })
                .collect(),
        }
    }

    fn find(&mut self, a: usize) -> State {
        let me = self.arr[a];
        if me.parent != a {
            let parent = self.find(me.parent);
            // a + da = p -- parent
            // p + dp = r -- root
            // a + (da + dp) = r -- change my parent to root
            let dist = me.dist + parent.dist;
            let rank = me.rank;
            let parent = parent.parent;
            self.arr[a] = State { dist, rank, parent };
        }
        self.arr[a]
    }

    fn union(&mut self, a: usize, b: usize, b_sub_a: i64) {
        let a = self.find(a);
        let b = self.find(b);
        if a.parent == b.parent {
            return;
        }
        // p: root of a
        // q: root of b
        match a.rank.cmp(&b.rank) {
            std::cmp::Ordering::Less => {
                // p has less depth => attach p as a child of q
                // a + da = p
                // b + db = q
                // a + b_sub_a = b
                // since we are attaching p to q, we need q - p
                // q - p = b + db - a - da
                //       = (b - a) + db - da
                let dist = b_sub_a + b.dist - a.dist;
                let parent = b.parent;
                let rank = a.rank;
                self.arr[a.parent] = State { dist, parent, rank };
            }
            std::cmp::Ordering::Greater => {
                // similar, but changed variables
                let dist = -b_sub_a - b.dist + a.dist;
                let parent = a.parent;
                let rank = b.rank;
                self.arr[b.parent] = State { dist, parent, rank };
            }
            std::cmp::Ordering::Equal => {
                // can pick up either side
                // attach p to q
                // then the rank of q increases
                let dist = b_sub_a + b.dist - a.dist;
                let parent = b.parent;
                let rank = a.rank;
                self.arr[a.parent] = State { dist, parent, rank };
                self.arr[b.parent].rank += 1;
            }
        }
    }

    fn query(&mut self, a: usize, b: usize) -> Option<i64> {
        let a = self.find(a);
        let b = self.find(b);
        if a.parent != b.parent {
            None
        } else {
            // a + da = r
            // b + db = r
            // b - a = da - db
            Some(a.dist - b.dist)
        }
    }
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut ans: Vec<String> = vec![];
    loop {
        let mut line = lines.next().unwrap().split_ascii_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let m: usize = line.next().unwrap().parse().unwrap();
        if n == 0 {
            break;
        }
        let mut set = DisjointSet::new(n);
        for line in lines.take(m) {
            let mut line = line.split_ascii_whitespace();
            if line.next().unwrap() == "!" {
                let a: usize = line.next().unwrap().parse().unwrap();
                let b: usize = line.next().unwrap().parse().unwrap();
                let a = a - 1;
                let b = b - 1;
                let da: i64 = line.next().unwrap().parse().unwrap();
                set.union(a, b, da);
            } else {
                let a: usize = line.next().unwrap().parse().unwrap();
                let b: usize = line.next().unwrap().parse().unwrap();
                let a = a - 1;
                let b = b - 1;
                if let Some(da) = set.query(a, b) {
                    ans.push(da.to_string())
                } else {
                    ans.push("UNKNOWN".to_string())
                }
            }
        }
    }
    ans.join("\n")
}

#[cfg(test)]
mod p03830_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "2 2
! 1 2 1
? 1 2
2 2
! 1 2 1
? 2 1
4 7
! 1 2 100
? 2 3
! 2 3 100
? 2 3
? 1 3
! 4 3 150
? 4 1
0 0"
        .to_string();
        let output = "1
-1
UNKNOWN
100
200
-50"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
