use std::io::{self, Read, Write};

struct DisjointSet {
    nums: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> DisjointSet {
        DisjointSet {
            nums: (0..n + 1).collect(),
        }
    }
    fn find(&self, num: usize) -> usize {
        let found = self.nums[num];
        if found == num {
            num
        } else {
            self.find(found)
        }
    }
    fn union(&mut self, n: usize, m: usize) {
        let first = self.find(n);
        let second = self.find(m);
        if first == second {
            return;
        }
        self.nums[first] = second;
    }
}

fn main() {
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut input).unwrap();

    let lines = &mut input.lines();

    let num_gates: usize = lines.next().unwrap().parse().unwrap();

    let mut disjoint_set = DisjointSet::new(num_gates + 1);

    let num_planes: usize = lines.next().unwrap().parse().unwrap();

    let mut ans = 0;

    for _ in 0..num_planes {
        let gate: usize = lines.next().unwrap().parse().unwrap();

        let gate = disjoint_set.find(gate);
        if gate == 0 {
            break;
        }

        disjoint_set.union(gate, gate - 1);
        ans += 1;
    }
    writeln!(output, "{}", ans).unwrap();
}
