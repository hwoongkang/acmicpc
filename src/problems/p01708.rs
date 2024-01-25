use std::ops::Sub;

#[derive(PartialOrd, Ord, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2D(i64, i64);

impl Sub for Vec2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Vec2D {
    fn _from(s: &str) -> Self {
        let mut words = s.split_whitespace();
        let x = words.next().unwrap().parse().unwrap();
        let y = words.next().unwrap().parse().unwrap();
        Self(x, y)
    }

    fn _cross(&self, rhs: &Self) -> i64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }

    fn _dot(&self, rhs: &Self) -> i64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    fn _norm(&self) -> i64 {
        self._dot(self)
    }
}

pub fn _solve(input: String) -> String {
    let points: Vec<Vec2D> = input.lines().skip(1).map(Vec2D::_from).collect();

    let len = points.len();

    let mut on_hull: Vec<bool> = (0..len).map(|_| false).collect();

    let mut left_most = 0;

    for (i, v) in points.iter().enumerate().skip(1) {
        let prev = &points[left_most];

        if v < prev {
            left_most = i;
        }
    }

    on_hull[left_most] = true;

    let mut curr = left_most;

    loop {
        let mut next: Option<usize> = None;
        let mut candidates: Vec<usize> = vec![];

        for i in 0..len {
            if i == curr {
                continue;
            }
            let me = points[curr];
            match next {
                None => {
                    next = Some(i);
                    continue;
                }
                Some(j) => {
                    let prev = points[j];
                    let prev = prev - me;
                    let curr = points[i] - me;
                    if prev._cross(&curr) > 0 {
                        next = Some(i);
                        candidates.clear();
                    } else if prev._cross(&curr) == 0 {
                        next = Some(i);
                        candidates.push(i);
                        if candidates.len() == 1 {
                            candidates.push(j);
                        }
                    }
                }
            }
        }

        let next = next.unwrap();

        if candidates.len() == 0 {
            if on_hull[next] {
                break;
            } else {
                on_hull[next] = true;
                curr = next;
            }
        } else {
            let mut farthest = (0i64, 0usize);
            for i in candidates.iter() {
                let i = *i;
                let dist = (points[i] - points[curr])._norm();
                if dist > farthest.0 {
                    farthest = (dist, i);
                }
            }
            curr = farthest.1;
            if on_hull[curr] {
                break;
            } else {
                on_hull[curr] = true;
            }
        }
    }

    on_hull
        .into_iter()
        .filter_map(|b| if b { Some(()) } else { None })
        .count()
        .to_string()
}

#[cfg(test)]
mod p01708_tests {
    use super::*;

    #[test]
    fn test_vec() {
        let a = Vec2D(0, 0);
        let b = Vec2D(0, 1);
        let c = Vec2D(1, 1);
        assert!(a < b);
        assert!(b < c);
    }

    #[test]
    fn test_part_1() {
        let input = "8
1 1
1 2
1 3
2 1
2 2
2 3
3 1
3 2"
        .to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }
}
