use std::collections::BinaryHeap;
pub fn _solve(input: String) -> String {
    let mut last_day = 0;
    let mut dues_rewards: Vec<(usize, usize)> = input
        .lines()
        .skip(1)
        .map(|l| {
            let mut words = l.split_ascii_whitespace();
            let due = words.next().unwrap().parse().unwrap();
            let reward = words.next().unwrap().parse().unwrap();
            last_day = last_day.max(due);
            (due, reward)
        })
        .collect();
    dues_rewards.sort_by_key(|dr| dr.0);

    let mut today = last_day + 1;

    let mut count = 0;

    let mut cursor = 0;

    let l = dues_rewards.len();

    let mut heap = BinaryHeap::<usize>::new();

    while today > 1 {
        today -= 1;

        while cursor < l {
            let rev = l - cursor - 1;
            let (due, reward) = dues_rewards[rev];

            if due >= today {
                cursor += 1;
                heap.push(reward)
            } else {
                break;
            }
        }

        if let Some(reward) = heap.pop() {
            count += reward;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod p01781_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "7
1 6
1 7
3 2
3 1
2 4
2 5
6 1"
        .to_string();
        let output = "15".to_string();
        assert_eq!(_solve(input), output);
    }
}
