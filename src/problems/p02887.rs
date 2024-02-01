#[derive(Debug)]
struct _Pos3D(i32, i32, i32);

impl _Pos3D {
    fn _dist(&self, rhs: &Self) -> i32 {
        let x = (self.0 - rhs.0).abs();
        let y = (self.1 - rhs.1).abs();
        let z = (self.2 - rhs.2).abs();
        x.min(y).min(z)
    }
}

fn _parse(line: &str) -> _Pos3D {
    let mut words = line.split_ascii_whitespace();
    _Pos3D(
        words.next().unwrap().parse().unwrap(),
        words.next().unwrap().parse().unwrap(),
        words.next().unwrap().parse().unwrap(),
    )
}

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let mut planets: Vec<(usize, _Pos3D)> = lines.take(n).map(_parse).enumerate().collect();

    let mut edges: Vec<Vec<(usize, i32)>> = vec![vec![(0, i32::MAX); 6]; n];

    planets.sort_by_key(|e| e.1 .0);

    for i in 1..n {
        let (a, pa) = &planets[i - 1];
        let (b, pb) = &planets[i];
        let dist = (pa.0 - pb.0).abs();

        let a = *a;
        let b = *b;

        edges[a][0] = (b, dist);
        edges[b][1] = (a, dist);
    }
    planets.sort_by_key(|e| e.1 .1);

    for i in 1..n {
        let (a, pa) = &planets[i - 1];
        let (b, pb) = &planets[i];
        let dist = (pa.1 - pb.1).abs();

        let a = *a;
        let b = *b;

        edges[a][2] = (b, dist);
        edges[b][3] = (a, dist);
    }
    planets.sort_by_key(|e| e.1 .2);

    for i in 1..n {
        let (a, pa) = &planets[i - 1];
        let (b, pb) = &planets[i];
        let dist = (pa.2 - pb.2).abs();

        let a = *a;
        let b = *b;

        edges[a][4] = (b, dist);
        edges[b][5] = (a, dist);
    }

    let mut on_mst = vec![false; n];
    on_mst[0] = true;

    let mut min_to_conn = vec![i32::MAX; n];

    let update_min = |vertex: usize, min_to_conn: &mut [i32], on_mst: &mut [bool]| {
        for &(to, cost) in edges[vertex].iter() {
            let to = to;
            if on_mst[to] {
                continue;
            }
            min_to_conn[to] = min_to_conn[to].min(cost);
        }
    };

    update_min(0, &mut min_to_conn, &mut on_mst);

    let mut ans = 0;

    for _ in 1..n {
        let (vertex, cost) = min_to_conn.iter().enumerate().min_by_key(|e| e.1).unwrap();
        ans += cost;
        on_mst[vertex] = true;
        min_to_conn[vertex] = i32::MAX;

        update_min(vertex, &mut min_to_conn, &mut on_mst);
    }

    ans.to_string()
}

#[cfg(test)]
mod p02887_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "5
11 -15 -15
14 -5 -15
-1 -1 -5
10 -4 -1
19 -4 19"
            .to_string();
        let output = "4".to_string();
        assert_eq!(_solve(input), output);
    }
}
