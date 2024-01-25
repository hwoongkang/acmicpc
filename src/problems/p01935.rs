pub fn _solve(input: String) -> String {
    let mut nums = [0f64; 26];
    let lines = &mut input.lines();
    let eq = lines.skip(1).next().unwrap();

    lines
        .enumerate()
        .for_each(|(i, s)| nums[i] = s.parse().unwrap());

    let mut stack: Vec<f64> = vec![];

    for ch in eq.chars() {
        match ch {
            '*' => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs * rhs);
            }
            '+' => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs + rhs);
            }
            '-' => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs - rhs);
            }
            '/' => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs / rhs);
            }
            ch => {
                let index = (ch as u32 - 'A' as u32) as usize;
                let num = nums[index];
                stack.push(num);
            }
        }
    }

    format!("{:.2}", stack.pop().unwrap())
}

#[cfg(test)]
mod p01935_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5
ABC*+DE/-
1
2
3
4
5"
        .to_string();
        let output = "6.20".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_2() {
        let input = "1
AA+A+
1"
        .to_string();
        let output = "3.00".to_string();
        assert_eq!(_solve(input), output);
    }
}
