use std::collections::HashMap;

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let target: i32 = lines.next().unwrap().parse().unwrap();
    let a: Vec<i32> = lines
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();
    let b: Vec<i32> = lines
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|w| w.parse().unwrap())
        .collect();

    fn sum_sub(arr: &[i32]) -> HashMap<i32, usize> {
        let mut temp = 0;
        let mut sarr = vec![temp];
        for n in arr.iter() {
            temp += n;
            sarr.push(temp);
        }

        let mut ans = HashMap::new();
        let l = sarr.len();
        for i in 0..(l - 1) {
            for j in (i + 1)..l {
                let now = sarr[j] - sarr[i];
                match ans.get_mut(&now) {
                    None => {
                        ans.insert(now, 1);
                    }
                    Some(ptr) => {
                        *ptr += 1;
                    }
                }
            }
        }
        ans
    }
    let sa = sum_sub(&a);
    let sb = sum_sub(&b);

    let ans = sa
        .iter()
        .map(|(key_a, count_a)| {
            let target = target - key_a;
            if let Some(count_b) = sb.get(&target) {
                count_a * count_b
            } else {
                0
            }
        })
        .sum::<usize>();

    ans.to_string()
}

#[cfg(test)]
mod p02143_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "5
4
1 3 1 2
3
1 3 2"
            .to_string();
        let output = "7".to_string();
        assert_eq!(_solve(input), output);
    }
}
