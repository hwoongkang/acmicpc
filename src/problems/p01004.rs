fn _is_in(cx: i32, cy: i32, r: i32, x: i32, y: i32) -> bool {
    let dx = x - cx;
    let dy = y - cy;
    r * r > dx * dx + dy * dy
}
fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let num_cases: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut answers: Vec<usize> = vec![];
    for _ in 0..num_cases {
        let nums: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|w| w.parse().unwrap())
            .collect();
        let x1 = nums[0];
        let y1 = nums[1];
        let x2 = nums[2];
        let y2 = nums[3];
        let num_planets: usize = lines.next().unwrap().trim().parse().unwrap();
        let mut ans = 0;
        for _ in 0..num_planets {
            let nums: Vec<i32> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
            let cx = nums[0];
            let cy = nums[1];
            let r = nums[2];
            let s = _is_in(cx, cy, r, x1, y1);
            let e = _is_in(cx, cy, r, x2, y2);
            if s != e {
                ans += 1;
            }
        }
        answers.push(ans);
    }
    answers
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests_p1004 {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = "2
-5 1 12 1
7
1 1 8
-3 -1 1
2 2 2
5 5 1
-4 5 1
12 1 1
12 1 2
-5 1 5 1
1
0 0 2"
            .to_string();
        let test_output = "3
0"
        .to_string();
        assert_eq!(_solve(test_input), test_output);
    }
    #[test]
    fn test_part_2() {
        let test_input = "3
-5 1 5 1
3
0 0 2
-6 1 2
6 2 2
2 3 13 2
8
-3 -1 1
2 2 3
2 3 1
0 1 7
-4 5 1
12 1 1
12 1 2
12 1 3
102 16 19 -108
12
-107 175 135
-38 -115 42
140 23 70
148 -2 39
-198 -49 89
172 -151 39
-179 -52 43
148 42 150
176 0 10
153 68 120
-56 109 16
-187 -174 8"
            .to_string();
        let test_output = "2
5
3"
        .to_string();
        assert_eq!(_solve(test_input), test_output);
    }
}
