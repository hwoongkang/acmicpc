pub fn _solve(_input: String) -> String {
    String::from(
        r#"         ,r'"7
r`-_   ,'  ,/
 \. ". L_r'
   `~\/
      |
      |"#,
    )
}

#[cfg(test)]
mod p25083_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "".to_string();
        let output = "".to_string();
        assert_eq!(_solve(input), output);
    }
}
