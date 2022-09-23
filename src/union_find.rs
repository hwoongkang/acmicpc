pub struct DisjointSet {
    pub nums: Vec<usize>,
    pub ranks: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> DisjointSet {
        DisjointSet {
            nums: (0..n + 1).collect(),
            ranks: (0..n + 1).map(|_| 1).collect(),
        }
    }
    pub fn find(&self, num: usize) -> usize {
        let found = self.nums[num];
        if found == num {
            num
        } else {
            self.find(found)
        }
    }
    pub fn union(&mut self, n: usize, m: usize) {
        let first = self.find(n);
        let second = self.find(m);
        if first == second {
            return;
        }
        let (first, second) = if self.ranks[first] > self.ranks[second] {
            (second, first)
        } else {
            (first, second)
        };
        self.nums[first] = second;
        if self.ranks[first] == self.ranks[second] {
            self.ranks[second] += 1;
        }
    }
    pub fn query(&self, n: usize, m: usize) -> bool {
        self.find(n) == self.find(m)
    }
}
