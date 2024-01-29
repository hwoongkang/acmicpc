import sys
import os

num = sys.argv[1]

num = num.zfill(5)

file = f"src/problems/p{num}.rs"

if os.path.isfile(file):
    print(f"already exists: {num}")
    sys.exit()

with open(file, 'w') as f:
    f.write(f"""
pub fn _solve(input: String) -> String {{
    String::new()
}}
            
#[cfg(test)]
mod p{num}_tests {{
    use super::*;

    #[test]
    fn test_case_1() {{
        let input = "".to_string();
        let output = "".to_string();
        assert_eq!(_solve(input), output);
    }}
}}
""")

with open("src/problems/mod.rs", "a") as f:
    f.write(f"pub mod p{num};")


print(sys.argv[1])