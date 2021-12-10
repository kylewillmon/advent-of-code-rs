use std::collections::HashMap;

use anyhow::Result;
use aoclib::strtools;
use regex::Regex;
use itertools::Itertools;

pub fn part1(input: String) -> Result<usize> {
    let (rules, messages) = strtools::split_once(input.as_str(), "\n\n");
    let rule = build_regex(rules, "0");
    let rule = Regex::new(format!("^{}$", rule).as_str()).unwrap();

    Ok(messages.trim().lines()
        .filter(|&msg| rule.is_match(msg))
        .count()
    )
}

pub fn part2(input: String) -> Result<usize> {
    let (rules, messages) = strtools::split_once(input.as_str(), "\n\n");
    let rule42 = build_regex(rules, "42");
    let rule42 = Regex::new(format!("^{}", rule42).as_str()).unwrap();

    let rule31 = build_regex(rules, "31");
    let rule31 = Regex::new(format!("^{}", rule31).as_str()).unwrap();

    Ok(messages.trim().lines()
        .filter(|msg| {
            let mut msg = *msg;
            let mut count = 0i64;
            let mut match31 = false;
            while let Some(_match) = rule42.find(msg) {
                msg = &msg[_match.end()..];
                count += 1;
            }
            while let Some(_match) = rule31.find(msg) {
                match31 = true;
                msg = &msg[_match.end()..];
                count -= 1;
            }
            msg.is_empty() && match31 && count > 0
        })
        .count()
    )
}

fn _build_regex_str(map: &HashMap<&str, &str>, rule: &str) -> String {
    let rule = map.get(rule).expect("rule not found!");
    if let Some(c) = rule.strip_prefix('"').and_then(|r| r.chars().next()) {
        return c.to_string();
    }

    let regex: Vec<String> = rule.split('|').map(|chain| {
        let mut regex = String::from("");
        let chain = chain.trim();
        for rule in chain.split(' ') {
            let rule = rule.trim();
            regex.push_str(_build_regex_str(map, rule).as_str());
        }
        regex
    }).collect();

    if regex.len() == 1 {
        regex.into_iter().next().unwrap()
    } else {
        format!("({})", regex.into_iter().join("|"))
    }
}

fn build_regex(input: &str, root: &str) -> String {
    let mut map = HashMap::new();

    for line in input.lines() {
        let (name, rule) = strtools::split_once(line, ":");
        let name = name.trim();
        let rule = rule.trim();
        map.insert(name, rule);
    }

    _build_regex_str(&map, root)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn part1_example() {
        assert_eq!(2, part1(EXAMPLE.to_string()).unwrap());
    }

    const EXAMPLE2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;


    #[test]
    fn part2_example() {
        assert_eq!(12, part2(EXAMPLE2.to_string()).unwrap());
    }
}