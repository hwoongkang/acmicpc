fn gcd(a: usize, b: usize) -> usize {
    let (a, b) = if a < b { (a, b) } else { (b, a) };
    if a == 0 {
        b
    } else if b % a == 0 {
        a
    } else {
        gcd(b % a, a)
    }
}
#[allow(clippy::needless_collect)]
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let num_lens: Vec<&str> = lines.take(n).collect();

    let k: usize = lines.next().unwrap().trim().parse().unwrap();

    let num_lens: Vec<(usize, usize)> = num_lens
        .into_iter()
        .map(|line| {
            let line = line.trim();
            let len = line.len();
            let mut rem = 0;
            for char in line.chars() {
                let num = char.to_digit(10).unwrap() as usize;
                rem *= 10;
                rem += num;
                rem %= k;
            }
            (rem, len)
        })
        .collect();

    let mut tens = [0; 51];
    let mut power = 1;
    for rem in &mut tens {
        let ans = power % k;

        *rem = ans;

        power = ans;
        power *= 10;
    }

    // dp: 2차원 배열
    // 1st index: bit masking
    // 2nd index: rem by K
    // value: count
    let mut visited: Vec<bool> = (0..(1 << 16)).map(|_| false).collect();

    let mut dp: Vec<Vec<usize>> = (0..(1 << 16)).map(|_| vec![0; k]).collect();

    for (i, (num, _len)) in num_lens.iter().enumerate() {
        let flag = 1 << i;
        visited[flag] = true;
        let rem = num % k;
        dp[flag][rem] = 1;
    }

    fn recursive(
        masking: usize,
        n: usize,
        k: usize,
        cache: &mut Vec<Vec<usize>>,
        visited: &mut [bool],
        num_lens: &[(usize, usize)],
        tens: &[usize],
    ) {
        if !visited[masking] {
            let mut rems = vec![0; k];
            for i in 0..n {
                // 구성하는 각각의 1에 대해 iteration
                let flag = 1 << i;

                if (flag & masking) == 0 {
                    continue;
                }
                let prev = masking & (!flag);

                // 일단 캐시를 채우고
                recursive(prev, n, k, cache, visited, num_lens, tens);

                for (rem, count) in cache[prev].iter().enumerate() {
                    let (num, len) = num_lens[i];

                    let num = rem * tens[len] + num;

                    let new_rem = num % k;
                    rems[new_rem] += count;
                }
            }
            for (rem, count) in rems.iter().enumerate() {
                cache[masking][rem] += count;
            }

            visited[masking] = true;
        }
    }
    let mut full_mask = 0;
    for i in 0..n {
        full_mask |= 1 << i;
    }

    recursive(full_mask, n, k, &mut dp, &mut visited, &num_lens, &tens);

    let ans = &dp[full_mask];

    let num = ans[0];
    let den = ans.iter().sum::<usize>();

    let d = gcd(num, den);

    format!("{}/{}", num / d, den / d)
}

#[cfg(test)]
mod p01086_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
3
2
1
2"
        .to_string();
        let output = "1/3".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "5
10
100
1000
10000
100000
10"
        .to_string();
        let output = "1/1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "5
11
101
1001
10001
100001
10"
        .to_string();
        let output = "0/1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "9
13
10129414190271203
102
102666818896
1216
1217
1218
101278001
1000021412678412681
21"
        .to_string();
        let output = "5183/36288".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_5() {
        let input = "15
11111111111111111111111111111111111111111111111111
22222222222222222222222222222222222222222222222222
33333333333333333333333333333333333333333333333333
44444444444444444444444444444444444444444444444444
55555555555555555555555555555555555555555555555555
11111111111111111111111111111111111111111111111111
22222222222222222222222222222222222222222222222222
33333333333333333333333333333333333333333333333333
44444444444444444444444444444444444444444444444444
55555555555555555555555555555555555555555555555555
66666666666666666666666666666666666666666666666666
77777777777777777777777777777777777777777777777777
88888888888888888888888888888888888888888888888888
99999999999999999999999999999999999999999999999999
43623423463454352346427347645623463463454523452300
100"
        .to_string();
        let output = "1/15".to_string();
        assert_eq!(_solve(input), output);
    }
}
