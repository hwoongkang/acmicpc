type _Color = bool;

const _WHITE: _Color = false;
const _BLUE: _Color = true;
pub fn _solve(input: String) -> String {
    let mut paper: [[_Color; 128]; 128] = [[_WHITE; 128]; 128];
    let lines = &mut input.lines();
    let size: usize = lines.next().unwrap().parse().unwrap();

    for (r, line) in lines.enumerate() {
        for (c, char) in line.split_whitespace().enumerate() {
            paper[r][c] = char == "1";
        }
    }

    fn num_partitions(
        paper: &[[_Color; 128]; 128],
        size: usize,
        start: (usize, usize),
    ) -> (usize, usize) {
        let (r, c) = start;

        if size == 1 {
            if paper[r][c] {
                (0, 1)
            } else {
                (1, 0)
            }
        } else {
            let mut _whites = 0;
            let mut _blues = 0;
            let size = size / 2;
            for dr in 0..=1usize {
                for dc in 0..=1usize {
                    let start = (r + dr * size, c + dc * size);
                    let (w, b) = num_partitions(paper, size, start);
                    _whites += w;
                    _blues += b;
                }
            }
            if _whites == 0 {
                (0, 1)
            } else if _blues == 0 {
                (1, 0)
            } else {
                (_whites, _blues)
            }
        }
    }

    let (w, b) = num_partitions(&paper, size, (0, 0));
    format!("{}\n{}", w, b)
}

#[cfg(test)]
mod p02630_test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "8
1 1 0 0 0 0 1 1
1 1 0 0 0 0 1 1
0 0 0 0 1 1 0 0
0 0 0 0 1 1 0 0
1 0 0 0 1 1 1 1
0 1 0 0 1 1 1 1
0 0 1 1 1 1 1 1
0 0 1 1 1 1 1 1"
            .to_string();
        let output = "9
7"
        .to_string();
        assert_eq!(_solve(input), output);
    }
}
