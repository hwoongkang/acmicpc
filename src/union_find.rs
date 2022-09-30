pub struct DisjointSet {
    pub nums: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> DisjointSet {
        DisjointSet {
            nums: (0..n + 1).collect(),
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
        self.nums[first] = second;
    }
}
