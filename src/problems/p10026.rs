#[derive(PartialEq, Eq, Clone)]
enum Color {
    R,
    G,
    B,
    K,
}

impl Color {
    fn same_as(&self, rhs: &Self, is_color_blind: bool) -> bool {
        match (is_color_blind, self, rhs) {
            (_, Color::K, Color::K) => false,
            (_, a, b) if a == b => true,
            (true, Color::R, Color::G) => true,
            (true, Color::G, Color::R) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
struct Figure {
    pixels: Vec<Vec<Color>>,
}

impl Figure {
    fn from(lines: &mut dyn Iterator<Item = &str>) -> Self {
        Self {
            pixels: lines
                .map(|line| {
                    line.chars()
                        .map(|ch| match ch {
                            'R' => Color::R,
                            'G' => Color::G,
                            'B' => Color::B,
                            _ => unreachable!(),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    fn size(&self) -> (usize, usize) {
        (self.pixels.len(), self.pixels[0].len())
    }

    fn next(&self, pos: &(usize, usize), is_color_blind: bool) -> Vec<(usize, usize)> {
        let mut ans = vec![];
        let &(r, c) = pos;
        let me = &self.pixels[r][c];
        let (mr, mc) = self.size();
        if r > 0 {
            ans.push((r - 1, c))
        }
        if c > 0 {
            ans.push((r, c - 1))
        }
        if r + 1 < mr {
            ans.push((r + 1, c))
        }
        if c + 1 < mc {
            ans.push((r, c + 1))
        }
        ans.into_iter()
            .filter_map(|(r, c)| {
                let candidate = &self.pixels[r][c];
                if me.same_as(candidate, is_color_blind) {
                    Some((r, c))
                } else {
                    None
                }
            })
            .collect()
    }

    fn sections(&mut self, is_color_blind: bool) -> usize {
        let mut count = 0;
        let (mr, mc) = self.size();
        for r in 0..mr {
            for c in 0..mc {
                if self.pixels[r][c] == Color::K {
                    continue;
                }
                let mut stack = vec![(r, c)];
                while let Some((r, c)) = stack.pop() {
                    for (nr, nc) in self.next(&(r, c), is_color_blind) {
                        stack.push((nr, nc));
                    }
                    self.pixels[r][c] = Color::K;
                }
                count += 1;
            }
        }
        count
    }
}

pub fn _solve(input: String) -> String {
    let figure = Figure::from(&mut input.lines().skip(1));

    let rgb = figure.clone().sections(false);

    let blind = figure.clone().sections(true);

    format!("{} {}", rgb, blind)
}

#[cfg(test)]
mod p10026_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5
RRRBB
GGBBB
BBBRR
BBRRR
RRRRR"
            .to_string();
        let output = "4 3".to_string();
        assert_eq!(_solve(input), output);
    }
}
