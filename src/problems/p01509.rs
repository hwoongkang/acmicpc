fn is_palindrome(s: &[u8]) -> bool {
    let l = s.len();
    if l == 0 {
        return true;
    }
    let mut a = 0;
    let mut b = l - 1;
    while a < b {
        if s[a] != s[b] {
            return false;
        }
        a += 1;
        b -= 1;
    }

    true
}
pub fn _solve(input: String) -> String {
    let input = input.trim().as_bytes();
    let l = input.len();
    let mut cache = vec![vec![usize::MAX; l + 1]; l + 1];
    fn local_solution(s: &[u8], cache: &mut Vec<Vec<usize>>, start: usize, end: usize) -> usize {
        if cache[start][end] == usize::MAX {
            if is_palindrome(&s[start..end]) {
                cache[start][end] = 1;
            } else {
                cache[start][end] = (start + 1..end)
                    .map(|cut| {
                        let front = local_solution(s, cache, start, cut);
                        let end = local_solution(s, cache, cut, end);
                        front + end
                    })
                    .min()
                    .unwrap_or(0);
            }
        }
        cache[start][end]
    }

    local_solution(input, &mut cache, 0, l).to_string()
}

#[cfg(test)]
mod p01509_tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        let s = "aba";
        let s = s.as_bytes();
        assert!(is_palindrome(s));
        let s = "abba";
        let s = s.as_bytes();
        assert!(is_palindrome(s));
        let s = "abcba";
        let s = s.as_bytes();
        assert!(is_palindrome(s));
    }

    #[test]
    fn test_case_1() {
        let input = "AAAA".to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "BBCDDECAECBDABADDCEBACCCBDCAABDBADD".to_string();
        let output = "22".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "ABCDEFGH".to_string();
        let output = "8".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "QWERTYTREWQWERT".to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }
}
