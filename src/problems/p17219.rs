use std::collections::HashMap;

pub fn _solve(input: String) -> String {
    let lines = &mut input.lines();
    let num_passwords: usize = lines
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let dict: HashMap<&str, &str> = lines
        .take(num_passwords)
        .map(|line| {
            let mut words = line.split_whitespace();
            let key = words.next().unwrap();
            let val = words.next().unwrap();
            (key, val)
        })
        .collect();
    lines
        .map(|line| dict.get(line).unwrap().to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod p17219_tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "16 4
noj.am IU
acmicpc.net UAENA
startlink.io THEKINGOD
google.com ZEZE
nate.com VOICEMAIL
naver.com REDQUEEN
daum.net MODERNTIMES
utube.com BLACKOUT
zum.com LASTFANTASY
dreamwiz.com RAINDROP
hanyang.ac.kr SOMEDAY
dhlottery.co.kr BOO
duksoo.hs.kr HAVANA
hanyang-u.ms.kr OBLIVIATE
yd.es.kr LOVEATTACK
mcc.hanyang.ac.kr ADREAMER
startlink.io
acmicpc.net
noj.am
mcc.hanyang.ac.kr"
            .to_string();
        let output = "THEKINGOD
UAENA
IU
ADREAMER"
            .to_string();
        assert_eq!(_solve(input), output);
    }
}
