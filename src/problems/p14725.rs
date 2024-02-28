use std::collections::HashMap;

struct Node<'a> {
    children: HashMap<&'a str, Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    fn update(&mut self, vec: &[&'a str]) {
        if !vec.is_empty() {
            let name = vec[0];

            let child = self
                .children
                .entry(name)
                .or_insert_with(|| Box::new(Node::new()));

            child.update(&vec[1..]);
        }
    }

    fn print(&self, level: usize, to: &mut Vec<String>) {
        let prefix = "--".repeat(level);
        let mut entries: Vec<(&&str, &Box<Node<'_>>)> = self.children.iter().collect();
        entries.sort_by_key(|kv| kv.0);

        for (name, child) in entries.iter() {
            to.push(format!("{}{}", prefix, name));
            child.print(level + 1, to);
        }
    }
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut root = Node::new();
    for line in lines.skip(1) {
        let words: Vec<&str> = line.split_ascii_whitespace().skip(1).collect();
        root.update(&words);
    }
    let mut ans = vec![];
    root.print(0, &mut ans);
    ans.join("\n")
}

#[cfg(test)]
mod p14725_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
2 B A
4 A B C D
2 A C"
            .to_string();
        let output = "A
--B
----C
------D
--C
B
--A"
        .to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4
2 KIWI BANANA
2 KIWI APPLE
2 APPLE APPLE
3 APPLE BANANA KIWI"
            .to_string();
        let output = "APPLE
--APPLE
--BANANA
----KIWI
KIWI
--APPLE
--BANANA"
            .to_string();
        assert_eq!(_solve(input), output);
    }
}
