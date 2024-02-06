pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();
    let answers = (0..n).map(|_| _test_case(lines)).map(|a| a.to_string());
    answers.collect::<Vec<_>>().join("\n")
}

fn _test_case(lines: &mut std::str::Lines) -> u8 {
    let mbtis: Vec<_MBTI> = lines
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(_parse_mbti)
        .collect();
    if mbtis.len() >= 33 {
        0
    } else {
        let mut ans = 255;
        let l = mbtis.len();
        for i in 0..(l - 2) {
            for j in (i + 1)..(l - 1) {
                for k in (j + 1)..l {
                    let a = mbtis[i];
                    let b = mbtis[j];
                    let c = mbtis[k];
                    let dist = _mbti_dist(a, b) + _mbti_dist(b, c) + _mbti_dist(c, a);
                    ans = ans.min(dist)
                }
            }
        }
        ans
    }
}

type _MBTI = u8;

fn _mbti_dist(a: _MBTI, b: _MBTI) -> u8 {
    let mut diff = a ^ b;
    let mut ans = 0;
    for _ in 0..4 {
        ans += diff & 1;
        diff >>= 1;
    }
    ans
}

fn _parse_mbti(s: &str) -> _MBTI {
    let mut ans = 0;
    let predicate = ['I', 'S', 'T', 'P'];
    for (i, c) in s.chars().enumerate() {
        if predicate[i] == c {
            ans |= 1 << i;
        }
    }
    ans
}

#[cfg(test)]
mod p21736_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "3
3
ENTJ INTP ESFJ
4
ESFP ESFP ESFP ESFP
5
INFP INFP ESTP ESTJ ISTJ"
            .to_string();
        let output = "8
0
4"
        .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_dist() {
        let istj = _parse_mbti("ISTJ");
        let isfj = _parse_mbti("ISFJ");
        assert_eq!(_mbti_dist(istj, isfj), 1);
        let intp = _parse_mbti("INTP");
        let entj = _parse_mbti("ENTJ");
        assert_eq!(_mbti_dist(intp, entj), 2);
    }
}
