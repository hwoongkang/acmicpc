pub fn _solve(input: String) -> String {
    let mut words = input.split_ascii_whitespace();
    let str = words.next().unwrap();
    let radix: u32 = words.next().unwrap().parse().unwrap();
    u32::from_str_radix(str, radix).unwrap().to_string()
}

#[cfg(test)]
mod p02745_tests {
    use super::*;

    #[test]
    fn test_std() {
        let n = u32::from_str_radix("Z", 36).unwrap();
        assert_eq!(n, 35);
    }

    #[test]
    fn test_case_1() {
        let input = "ZZZZZ 36".to_string();
        let output = "60466175".to_string();
        assert_eq!(_solve(input), output);
    }
}
