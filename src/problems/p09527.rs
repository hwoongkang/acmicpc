pub fn _solve(input: String) -> String {
    let mut words = input.split_whitespace();
    let a: usize = words.next().unwrap().parse().unwrap();
    let b: usize = words.next().unwrap().parse().unwrap();
    let ans = _num_ones(b) - _num_ones(a - 1);
    ans.to_string()
}

fn _helper(n: usize, power: usize) -> usize {
    let me = 1 << power;
    let cycle = me << 1;
    let full_cycle = n / cycle;

    let ans = full_cycle * me;

    let rem = n % cycle;

    if rem >= me {
        ans + rem + 1 - me
    } else {
        ans
    }
}

fn _num_ones(n: usize) -> usize {
    let mut ans = 0;
    for power in 0.. {
        let temp = _helper(n, power);
        if temp == 0 {
            break;
        } else {
            ans += temp;
        }
    }
    ans
}

#[cfg(test)]
mod p09527_tests {
    use super::*;
    #[test]
    fn test_helper() {
        assert_eq!(_helper(30, 2), 15);
    }
    #[test]
    fn test_num_ones() {
        assert_eq!(_num_ones(12), 22);
        assert_eq!(_num_ones(1), 1);
        assert_eq!(_num_ones(0), 0);
    }

    #[test]
    fn test_part_1() {
        let input = "2 12".to_string();
        let output = "21".to_string();
        assert_eq!(_solve(input), output);
    }
}
