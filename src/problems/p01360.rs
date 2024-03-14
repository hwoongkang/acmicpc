struct State {
    history: Vec<(usize, String)>,
}

impl State {
    fn new() -> Self {
        Self {
            history: vec![(0, String::new())],
        }
    }
}

enum Command<'a> {
    Type(&'a str),
    Undo(usize),
}

impl State {
    fn apply(&mut self, cmd: Command<'_>, t: usize) {
        match cmd {
            Command::Type(str) => {
                let (_, value) = self.history.last().unwrap();
                self.history.push((t, value.clone() + str))
            }
            Command::Undo(dt) => {
                let t2 = t.saturating_sub(dt);
                let state = self
                    .history
                    .iter()
                    .rev()
                    .find(|(timestamp, _)| *timestamp < t2);
                let new_value = if let Some((_, value)) = state {
                    value.clone()
                } else {
                    String::new()
                };
                self.history.push((t, new_value))
            }
        }
    }
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut state = State::new();
    for line in lines.skip(1) {
        let mut words = line.split_ascii_whitespace();
        let cmd: Command = match words.next().unwrap() {
            "undo" => {
                let dt: usize = words.next().unwrap().parse().unwrap();
                Command::Undo(dt)
            }
            "type" => {
                let str = words.next().unwrap();
                Command::Type(str)
            }
            _ => unreachable!(),
        };
        let t: usize = words.next().unwrap().parse().unwrap();
        state.apply(cmd, t);
    }

    state.history.last().unwrap().1.clone()
}

#[cfg(test)]
mod p01360_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "4
type a 1
type b 2
type c 3
undo 3 5"
            .to_string();
        let output = "a".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4
type a 1
type b 2
undo 2 3
undo 2 4"
            .to_string();
        let output = "a".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "3
type a 1
undo 1 2
undo 1 3"
            .to_string();
        let output = "a".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "4
type a 1
type b 2
type c 3
undo 10 1000"
            .to_string();
        let output = "abc".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_5() {
        let input = "1
undo 1 1"
            .to_string();
        let output = "".to_string();
        assert_eq!(_solve(input), output);
    }
}
