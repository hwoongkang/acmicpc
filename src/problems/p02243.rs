const NUM_TASTES: usize = 1_000_000;
const POWER: usize = 1_048_576;

trait Unionable: Copy + Clone + Ord + ToString {
    fn union(&self, other: &Self) -> Self;
    fn inv(&self, other: &Self) -> Self;
    fn default() -> Self;
}

impl Unionable for i32 {
    fn union(&self, other: &i32) -> i32 {
        self + other
    }
    fn inv(&self, other: &i32) -> i32 {
        self - other
    }
    fn default() -> i32 {
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
        NUM_TASTES
    }

    fn update(&mut self, i: usize, val: T) {
        let i = (POWER + i) % NUM_TASTES;
        let n = NUM_TASTES;
        let mut i = n + i;
        self.data[i] = self.data[i].union(&val);
        while i > 1 {
            self.data[i >> 1] = self.data[i].union(&self.data[i ^ 1]);
            i >>= 1;
        }
    }

    fn query(&self, nth: T) -> usize {
        let mut nth = nth;
        let mut node = 1;
        while node < NUM_TASTES {
            let left = node << 1;
            let right = node << 1 | 1;
            let left_sum = self.data[left];

            if left_sum < nth {
                node = right;
                nth = nth.inv(&left_sum);
            } else {
                node = left;
            }
        }

        // let j = (POWER + i) % NUM_TASTES;
        // j = POWER + i - NUM_TASTES
        // j + NUM_TASTES = POWER + i
        let j = node - NUM_TASTES;
        let predicate = POWER - NUM_TASTES;
        if j >= predicate {
            j - predicate
        } else {
            let diff = NUM_TASTES - predicate;

            diff + j
        }
    }
}
pub fn _solve(input: String) -> String {
    let candies: Vec<i32> = vec![0; NUM_TASTES];
    let mut tree = SegmentTree::from(&candies);
    let mut ans: Vec<String> = vec![];
    for line in input.lines().skip(1) {
        let mut line = line.split_ascii_whitespace();

        if line.next().unwrap() == "1" {
            let nth: i32 = line.next().unwrap().parse().unwrap();
            let taste = tree.query(nth) + 1;
            ans.push(taste.to_string());
            tree.update(taste - 1, -1);
        } else {
            let taste: usize = line.next().unwrap().parse().unwrap();
            let num: i32 = line.next().unwrap().parse().unwrap();
            let taste = taste - 1;
            tree.update(taste, num);
        }
    }
    ans.join("\n")
}

#[cfg(test)]
mod p02243_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "6
2 1 2
2 3 3
2 4 3
1 2
1 1
2 3 -3
1 1"
        .to_string();
        let output = "1
1
4"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
