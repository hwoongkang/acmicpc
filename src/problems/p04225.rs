#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct _Vec2D(i64, i64);

impl std::ops::Sub for _Vec2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        _Vec2D(x, y)
    }
}

impl _Vec2D {
    fn _from(s: &str) -> Self {
        let mut words = s.split_ascii_whitespace();

        let x = words.next().unwrap().parse().unwrap();
        let y = words.next().unwrap().parse().unwrap();

        _Vec2D(x, y)
    }

    fn _cross(&self, rhs: &Self) -> i64 {
        self.0 * rhs.1 - self.1 * rhs.0
    }

    fn _dot(&self, rhs: &Self) -> i64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    fn _size(&self) -> f64 {
        (self._dot(self) as f64).sqrt()
    }
}

fn _distance(point: _Vec2D, line_a: _Vec2D, line_b: _Vec2D) -> f64 {
    let line_vec = line_b - line_a;
    let p = point - line_a;
    let cross = line_vec._cross(&p);
    let ans = (cross as f64) / line_vec._size();
    ans.abs()
}

fn _filter_convex(points: &[_Vec2D]) -> Vec<_Vec2D> {
    let len = points.len();
    let mut on_hull: Vec<bool> = (0..len).map(|_| false).collect();
    let mut left_most = 0;
    for (i, v) in points.iter().enumerate().skip(1) {
        let prev = &points[left_most];

        if v < prev {
            left_most = i;
        }
    }
    let mut ans = vec![points[left_most]];

    on_hull[left_most] = true;
    let mut curr = left_most;

    loop {
        let mut next: Option<usize> = None;
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
                    } else if prev._cross(&curr) == 0 {
                        let prev_size = prev._size();
                        let curr_size = curr._size();
                        next = Some(if prev_size > curr_size { j } else { i });
                    }
                }
            }
        }
        let next = next.unwrap();
        if on_hull[next] {
            break;
        } else {
            on_hull[next] = true;
            curr = next;
            ans.push(points[next]);
        }
    }
    ans
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut ans: Vec<f64> = vec![];
    loop {
        let num_vertices: usize = lines.next().unwrap().parse().unwrap();
        if num_vertices == 0 {
            break;
        }
        let points: Vec<_Vec2D> = lines.take(num_vertices).map(_Vec2D::_from).collect();

        let points = _filter_convex(&points);

        let len = points.len();

        let mut min = f64::MAX;
        for i in 0..len {
            let j = (i + 1) % len;

            let mut max = f64::MIN;
            for k in 2..len {
                let k = (i + k) % len;
                let dist = _distance(points[k], points[i], points[j]);
                max = max.max(dist);
            }
            min = min.min(max);
        }

        ans.push(min);
    }
    let ans: Vec<String> = ans
        .into_iter()
        .enumerate()
        .map(|(i, f)| {
            let n = f * 100.0;
            let n = n.ceil();
            let n = n / 100.0;

            format!("Case {}: {:.2}", i + 1, n)
        })
        .collect();
    ans.join("\n")
}

#[cfg(test)]
mod p04225_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
0 0
3 0
0 4
4
0 10
10 0
20 10
10 20
0"
        .to_string();
        let output = "Case 1: 2.40
Case 2: 14.15"
            .to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_2() {
        let input = "4
0 0
2 0
3 1
1 2
0"
        .to_string();
        let output = "Case 1: 2.00".to_string();
        assert_eq!(_solve(input), output);
    }
}
