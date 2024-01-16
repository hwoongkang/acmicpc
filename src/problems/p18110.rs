pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let n: i32 = lines.next().unwrap().parse().unwrap();
    let mut nums: Vec<i32> = lines.map(|l| l.parse().unwrap()).collect();
    let exclude = _rounded_division(n * 3, 20);
    nums.sort();
    let m = n - 2 * exclude;
    if m == 0 {
        return 0.to_string();
    }
    let sum: i32 = nums
        .into_iter()
        .skip(exclude as usize)
        .take(m as usize)
        .sum();
    _rounded_division(sum, m).to_string()
}

fn _rounded_division(divisor: i32, divider: i32) -> i32 {
    (divisor + (divider / 2)) / divider
}

#[cfg(test)]
mod p18110_test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "5
1
5
5
7
8"
        .to_string();
        let output = "6".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_part_2() {}
}
