struct Room {
    dist_a: i32,
    dist_b: i32,
    qty: i32,
}
pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let mut ans: Vec<String> = vec![];
    loop {
        let mut line = lines.next().unwrap().split_ascii_whitespace();
        let num_rooms: usize = line.next().unwrap().parse().unwrap();
        let mut a_cap: i32 = line.next().unwrap().parse().unwrap();
        let mut b_cap: i32 = line.next().unwrap().parse().unwrap();
        if num_rooms == 0 {
            break;
        }
        let mut rooms: Vec<Room> = lines
            .take(num_rooms)
            .map(|line| {
                let mut line = line.split_ascii_whitespace();
                let qty: i32 = line.next().unwrap().parse().unwrap();
                let dist_a: i32 = line.next().unwrap().parse().unwrap();
                let dist_b: i32 = line.next().unwrap().parse().unwrap();
                Room {
                    qty,
                    dist_a,
                    dist_b,
                }
            })
            .collect();
        rooms.sort_by_key(|room| room.dist_a - room.dist_b);

        let mut by_a = vec![0; num_rooms];

        for i in 0..num_rooms {
            let room = &rooms[i];
            let load = a_cap.min(room.qty);
            a_cap -= load;
            by_a[i] += load;
            if a_cap == 0 {
                break;
            }
        }

        for i in (0..num_rooms).rev() {
            let room = &rooms[i];
            if by_a[i] == 0 {
                b_cap -= room.qty;
            } else {
                let mandatory = room.qty - by_a[i];
                b_cap -= mandatory;
                if room.dist_a > room.dist_b {
                    let load = by_a[i].min(b_cap);
                    by_a[i] -= load;
                    b_cap -= load;
                }
            }
            if b_cap == 0 {
                break;
            }
        }

        let mut local_ans = 0;

        for (room, by_a) in rooms.into_iter().zip(by_a.into_iter()) {
            let by_b = room.qty - by_a;
            local_ans += room.dist_a * by_a;
            local_ans += room.dist_b * by_b;
        }
        ans.push(local_ans.to_string())
    }
    ans.join("\n")
}

#[cfg(test)]
mod p04716_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3 15 35
10 20 10
10 10 30
10 40 10
0 0 0"
            .to_string();
        let output = "300".to_string();
        assert_eq!(_solve(input), output);
    }
}
