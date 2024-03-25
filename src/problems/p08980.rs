#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Cargo {
    to: usize,
    from: usize,
    qty: usize,
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();

    let line = lines.next().unwrap();

    let mut line = line.split_ascii_whitespace();

    let n: usize = line.next().unwrap().parse().unwrap();

    let truck_cap: usize = line.next().unwrap().parse().unwrap();

    let mut cargos: Vec<Cargo> = lines
        .skip(1)
        .map(|line| {
            let mut words = line.split_ascii_whitespace();
            let from = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let to = words.next().unwrap().parse::<usize>().unwrap() - 1;
            let qty = words.next().unwrap().parse::<usize>().unwrap();
            Cargo { from, to, qty }
        })
        .collect();

    cargos.sort();

    let mut capacities = vec![truck_cap; n];

    let mut ans = 0;

    for Cargo { from, to, qty } in cargos {
        let mut load = qty;

        for cap in capacities.iter_mut().take(to).skip(from) {
            load = load.min(*cap);
        }

        ans += load;
        for cap in capacities.iter_mut().take(to).skip(from) {
            *cap -= load;
        }
    }

    ans.to_string()
}

#[cfg(test)]
mod p08980_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "4 40
6
3 4 20
1 2 10
1 3 20
1 4 30
2 3 10
2 4 20"
            .to_string();
        let output = "70".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4 40
6
3 4 20
1 2 10
1 3 20
1 4 30
2 3 10
2 4 20"
            .to_string();
        let output = "70".to_string();
        assert_eq!(_solve(input), output);
    }
}
