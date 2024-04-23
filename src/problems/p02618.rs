fn dist(lhs: (usize, usize), rhs: (usize, usize)) -> usize {
    let (x0, y0) = lhs;
    let (x1, y1) = rhs;
    let mut ans = 0;
    if x0 > x1 {
        ans += x0 - x1;
    } else {
        ans += x1 - x0;
    }
    if y0 > y1 {
        ans += y0 - y1;
    } else {
        ans += y1 - y0;
    }
    ans
}
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let w: usize = lines.next().unwrap().parse().unwrap();
    let mut cases = vec![((1, 1), (n, n))];
    lines
        .map(|line| {
            let words = &mut line.split_ascii_whitespace();
            let x: usize = words.next().unwrap().parse().unwrap();
            let y: usize = words.next().unwrap().parse().unwrap();
            (x, y)
        })
        .for_each(|pos| cases.push((pos, pos)));

    let mut dp = vec![vec![(usize::MAX, 1); w + 1]; w + 1];

    dp[0][0] = (0, 1);

    // k번째 case: 둘 중 하나의 경찰차는 k번째 위치에,
    // 나머지 하나는 0~k-1 번째 위치에 있다.
    // 즉 k+1 번째 사건이 들어올 때 가능했을 조합들은
    // (0, k), (1, k), ..., (k-1, k), (k, k-1), (k, k-2), ..., (k, 0)
    // 이 모든 조합들에 대해서, (x, k+1) 또는 (k+1, y)가 되었을 때를 min 취해주면 될듯.
    // (x, k+1)이 되었다면, 2번차가 배정된 것
    // (k+1, y)가 되었다면, 1번차가 배정된 것
    // 나중에 (x, w)가 최소였다고 하자.
    // (x, w)일 때 2번차가 배정

    for k in 0..w {
        let mut prevs = vec![];
        if k == 0 {
            prevs = vec![(0, 0)];
        } else {
            for i in 0..k {
                prevs.push((i, k));
            }
            for i in 0..k {
                prevs.push((k, i));
            }
        }

        for (x, y) in prevs {
            let first = dp[x][y].0 + dist(cases[x].0, cases[k + 1].0);
            if first < dp[k + 1][y].0 {
                dp[k + 1][y] = (first, x);
            }
            let second = dp[x][y].0 + dist(cases[y].1, cases[k + 1].1);
            if second < dp[x][k + 1].0 {
                dp[x][k + 1] = (second, y);
            }
        }
    }

    let mut ans = (usize::MAX, (0, 0));

    for i in 0..w {
        let dist = dp[i][w];
        if dist.0 < ans.0 {
            ans = (dist.0, (i, w));
        }
        let dist = dp[w][i];
        if dist.0 < ans.0 {
            ans = (dist.0, (w, i));
        }
    }

    let dist_ans = ans.0;
    let mut state = ans.1;
    let mut ans: Vec<String> = vec![];
    for i in (1..=w).rev() {
        let (x, y) = state;
        let (_, prev) = &dp[state.0][state.1];
        if x == i {
            ans.push("1".to_string());
            state = (*prev, y);
        } else {
            ans.push("2".to_string());
            state = (x, *prev);
        }
    }

    ans.push(dist_ans.to_string());
    ans.reverse();
    ans.join("\n")
}

#[cfg(test)]
mod p02618_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "6
3
3 5
5 5
2 3"
        .to_string();
        let output = "9
2
2
1"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
