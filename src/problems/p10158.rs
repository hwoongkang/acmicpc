fn _wrapping_add(now: i64, width: i64, t: i64) -> i64 {
    let remaining = width - now;
    if t <= remaining {
        return now + t;
    }
    let t = t - remaining;
    let laps = (t / width) % 2;
    let t = t % width;
    if laps == 0 {
        width - t
    } else {
        t
    }
}
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut line = lines.next().unwrap().split_ascii_whitespace();
    let w: i64 = line.next().unwrap().parse().unwrap();
    let h: i64 = line.next().unwrap().parse().unwrap();

    let mut line = lines.next().unwrap().split_ascii_whitespace();
    let p: i64 = line.next().unwrap().parse().unwrap();
    let q: i64 = line.next().unwrap().parse().unwrap();

    let t: i64 = lines.next().unwrap().parse().unwrap();

    format!("{} {}", _wrapping_add(p, w, t), _wrapping_add(q, h, t))
}

#[cfg(test)]
mod p10158_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "6 4
4 1
8"
        .to_string();
        let output = "0 1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "6 4
5 3
4"
        .to_string();
        let output = "3 1".to_string();
        assert_eq!(_solve(input), output);
    }
}
