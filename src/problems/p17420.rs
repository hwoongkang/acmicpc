#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Gifticon {
    use_on: usize,
    valid_until: usize,
}

fn days_helper(lhs: usize, rhs: usize) -> usize {
    if lhs <= rhs {
        0
    } else {
        let diff = lhs - rhs;
        let quo = (diff - 1) / 30;
        quo + 1
    }
}
pub fn _solve(input: String) -> String {
    let mut lines = input.lines();
    let valid_untils = lines.nth(1).unwrap().split_ascii_whitespace();
    let use_ons = lines.next().unwrap().split_ascii_whitespace();
    let mut count: usize = 0;
    let mut gifticons: Vec<Gifticon> = valid_untils
        .zip(use_ons)
        .map(|(w1, w2)| (w1.parse().unwrap(), w2.parse().unwrap()))
        .map(|(valid_until, use_on)| {
            let c = days_helper(use_on, valid_until);
            let valid_until = valid_until + c * 30;
            count += c;
            Gifticon {
                valid_until,
                use_on,
            }
        })
        .collect();

    gifticons.sort();

    let mut prev_max = Gifticon {
        use_on: 0,
        valid_until: 0,
    };

    let l = gifticons.len();

    let mut i = 0;
    while i < l {
        let a = gifticons[i];
        // 뭔가 잘못됐을 때
        if a.valid_until < prev_max.valid_until {
            // 그럼 지금부터 "같은 날짜에 사용하기로 한 것들"을 한 번 처리해주어야 함
            let mut j = i;
            let mut local_max = 0;
            while j < l {
                let mut b = gifticons[j];
                if a.use_on != b.use_on {
                    break;
                }
                let c = days_helper(prev_max.valid_until, b.valid_until);
                b.valid_until += c * 30;
                count += c;
                local_max = local_max.max(b.valid_until);

                j += 1;
            }
            prev_max = Gifticon {
                use_on: a.use_on,
                valid_until: local_max,
            };
            i = j;
        }
        // 쓸 수 있었을 때
        else {
            // 같은 날짜에 쓰기로 한 것들을 쭉 건너뛰면 됨
            let mut j = i;
            let mut local_max = 0;
            while j < l {
                let b = gifticons[j];
                if a.use_on != b.use_on {
                    break;
                }
                local_max = local_max.max(b.valid_until);
                j += 1;
            }
            prev_max = Gifticon {
                use_on: a.use_on,
                valid_until: local_max,
            };
            i = j;
        }
    }

    count.to_string()
}

#[cfg(test)]
mod p17420_tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = "3
10 5 4
10 100 30"
            .to_string();
        let output = "5".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = "4
24 2 3 29
25 30 30 30"
            .to_string();
        let output = "6".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = "3
60 60 60
90 90 90"
            .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_4() {
        let input = "4
39 8 10 0
40 60 60 90"
            .to_string();
        let output = "10".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_5() {
        let input = "1
1
10"
        .to_string();
        let output = "1".to_string();
        assert_eq!(_solve(input), output);
    }
    #[test]
    fn test_case_6() {
        let input = "1
50
30"
        .to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_7() {
        let input = "2
100 70
30 30"
            .to_string();
        let output = "0".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_8() {
        let input = "4
45 10 15 50
30 20 60 40
3"
        .to_string();
        let output = "3".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_9() {
        let input = "5
10 20 30 40 50
50 40 30 20 10
10"
        .to_string();
        let output = "10".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_10() {
        let input = "3
5 5 5
1000 10000 100000
3702"
            .to_string();
        let output = "3702".to_string();
        assert_eq!(_solve(input), output);
    }

    #[test]
    fn test_case_11() {
        let input = "100
41 449 328 474 150 709 467 329 936 440 700 117 258 811 952 491 993 931 823 431 359 590 899 153 292 370 404 698 699 876 442 705 757 527 868 893 642 273 18 885 675 788 291 303 656 660 126 704 225 862 522 617 630 725 17 847 715 732 502 778 304 32 168 841 288 76 31 934 245 626 419 782 875 723 346 335 992 70 369 545 610 611 60 935 738 829 962 369 918 282 928 407 602 312 532 517 102 80 907 525
86 7 94 65 80 32 39 84 60 65 72 61 58 84 8 72 12 19 47 49 49 59 71 52 34 22 21 20 92 33 80 39 74 9 28 97 100 93 29 25 4 66 79 81 98 21 91 62 82 4 59 100 34 1 51 80 92 69 77 39 38 97 51 34 35 19 22 1 67 9 90 31 82 11 51 84 78 70 74 42 100 88 53 80 57 62 32 51 48 63 92 46 4 61 31 98 69 52 88 20
3206"
            .to_string();
        let output = "3206".to_string();
        assert_eq!(_solve(input), output);
    }
}
