use std::{cell::RefCell, rc::Rc};

trait Unionable: Copy + Clone {
    type Delta;
    fn union(&self, rhs: &Self) -> Self;
    fn default() -> Self;
    fn apply(&self, delta: &Self::Delta) -> Self;
    fn get_delta(&self, rhs: &Self) -> Self::Delta;
}

impl Unionable for i64 {
    type Delta = (i64, i64);
    fn union(&self, rhs: &i64) -> i64 {
        self + rhs
    }
    fn default() -> i64 {
        0
    }
    fn apply(&self, delta: &(i64, i64)) -> i64 {
        self + delta.0 - delta.1
    }
    fn get_delta(&self, rhs: &Self) -> Self::Delta {
        (*rhs, *self)
    }
}

#[derive(Copy, Clone, Debug)]
struct Range(usize, usize);

impl Range {
    fn intersection(&self, rhs: &Self) -> Option<Range> {
        let start = self.0.max(rhs.0);
        let end = self.1.min(rhs.1);
        if start < end {
            Some(Range(start, end))
        } else {
            None
        }
    }

    fn intersects(&self, rhs: &Self) -> bool {
        self.0 < rhs.1 && rhs.0 < self.1
    }

    fn len(&self) -> usize {
        self.1 - self.0
    }
}

#[derive(Debug)]
struct SegmentTree<T>
where
    T: Unionable,
{
    range: Range,
    value: T,
    left: Option<Rc<RefCell<SegmentTree<T>>>>,
    right: Option<Rc<RefCell<SegmentTree<T>>>>,
}

impl<T> SegmentTree<T>
where
    T: Unionable,
{
    fn from(data: &[T]) -> Self {
        let range = Range(0, data.len());
        Self::new(range, data)
    }

    fn new(range: Range, data: &[T]) -> Self {
        let len = range.len();
        if len == 1 {
            Self {
                range,
                value: data[0],
                left: None,
                right: None,
            }
        } else {
            let len = (len + 1) / 2;

            let left = Range(range.0, range.0 + len);
            let right = Range(range.0 + len, range.1.min(range.0 + len + len));

            let left = Self::new(left, &data[0..len]);
            let right = Self::new(right, &data[len..]);

            let value = left.value.union(&right.value);

            let left = Rc::new(RefCell::new(left));
            let right = Rc::new(RefCell::new(right));

            Self {
                range,
                value,
                left: Some(left),
                right: Some(right),
            }
        }
    }

    fn is_leaf(&self) -> bool {
        self.range.len() == 1
    }

    fn query(&self, range: &Range) -> T {
        if self.is_leaf() {
            self.value
        } else {
            let mut ans = T::default();
            if let Some(left) = &self.left {
                if let Some(range) = left.borrow().range.intersection(range) {
                    ans = ans.union(&left.borrow().query(&range));
                }
            }
            if let Some(right) = &self.right {
                if let Some(range) = right.borrow().range.intersection(range) {
                    ans = ans.union(&right.borrow().query(&range));
                }
            }
            ans
        }
    }

    fn update(&mut self, index: usize, value: T) -> T::Delta {
        if self.is_leaf() {
            let original = self.value;
            self.value = value;
            original.get_delta(&value)
        } else {
            if let Some(left) = &self.left {
                if left.borrow().range.intersects(&Range(index, index + 1)) {
                    let delta = left.borrow_mut().update(index, value);
                    self.value = self.value.apply(&delta);
                    return delta;
                }
            }
            if let Some(right) = &self.right {
                if right.borrow().range.intersects(&Range(index, index + 1)) {
                    let delta = right.borrow_mut().update(index, value);
                    self.value = self.value.apply(&delta);
                    return delta;
                }
            }
            unreachable!()
        }
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

    let mut tree = SegmentTree::from(&nums);

    let mut ans: Vec<String> = vec![];

    for line in lines {
        let mut words = line.split_ascii_whitespace();

        if words.next().unwrap() == "1" {
            let index: usize = words.next().unwrap().parse().unwrap();
            let num: i64 = words.next().unwrap().parse().unwrap();
            tree.update(index - 1, num);
        } else {
            let from: usize = words.next().unwrap().parse().unwrap();
            let to: usize = words.next().unwrap().parse().unwrap();
            ans.push(tree.query(&Range(from - 1, to)).to_string());
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
