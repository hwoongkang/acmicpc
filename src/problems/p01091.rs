fn gcd(a: usize, b: usize) -> usize {
    if a > b {
        gcd(b, a)
    } else if b % a == 0 {
        a
    } else {
        gcd(b % a, a)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let d = gcd(a, b);
    a * b / d
}
struct Permutation {
    data: Vec<usize>,
}

impl Permutation {
    fn from(s: &str) -> Self {
        Self {
            data: s
                .split_ascii_whitespace()
                .map(|w| w.parse().unwrap())
                .collect(),
        }
    }
    fn size(&self) -> usize {
        self.data.len()
    }
    fn max_period(&self) -> usize {
        let mut visited: Vec<bool> = self.data.iter().map(|_| false).collect();
        let mut cycles: Vec<usize> = vec![];
        let l = self.size();
        for i in 0..l {
            if visited[i] {
                continue;
            }
            let mut num = i;
            let mut count = 0;
            loop {
                count += 1;
                num = self.data[num];
                visited[num] = true;
                if num == i {
                    cycles.push(count);
                    break;
                }
            }
        }
        cycles.into_iter().fold(1, lcm)
    }

    fn permute(&self, nums: Vec<usize>) -> Vec<usize> {
        let l = self.size();
        let mut new = vec![0; l];
        for (i, num) in nums.iter().enumerate().take(l) {
            let j = self.data[i];
            new[j] = *num;
        }

        new
    }
}
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut signature: Vec<usize> = lines
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|word| word.parse().unwrap())
        .collect();
    let permutation = Permutation::from(lines.next().unwrap());
    let answer: Vec<usize> = (0..permutation.size()).map(|n| n % 3).collect();
    for i in 0..permutation.max_period() {
        if signature == answer {
            return i.to_string();
        }
        signature = permutation.permute(signature);
    }
    "-1".to_string()
}

#[cfg(test)]
mod p01091_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
2 0 1
1 2 0"
            .to_string();
        let output = "2".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "6
0 1 2 0 1 2
1 4 0 3 2 5"
            .to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "3
1 0 2
0 2 1"
            .to_string();
        let output = "-1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "12
1 1 2 0 2 0 1 0 2 2 1 0
5 0 9 7 1 8 3 10 4 11 6 2"
            .to_string();
        let output = "59".to_string();
        assert_eq!(_solve(input), output);
    }
}
