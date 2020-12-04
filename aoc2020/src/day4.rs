use std::str::FromStr;
use std::collections::{HashSet, HashMap};

struct Passport(HashMap<String, String>);

impl Passport {
    fn validate_keys(&self) -> bool
    {
        let required_keys: HashSet<String> = [
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
        ].iter().cloned().map(|s| s.to_string()).collect();

        let keys: HashSet<String> = self.0.keys().cloned().collect();

        keys.is_superset(&required_keys)
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let mut items = HashMap::new();
        for i in s.split_ascii_whitespace() {
            let mut split = i.splitn(2, ':');
            let key = split.next().unwrap().to_string();
            let val = split.next().ok_or("no value")?.to_string();

            items.insert(key, val);
        }

        Ok(Passport(items))
    }
}

pub fn part1(input: String) -> usize
{
    // Beware the CRLF
    let input = input.replace("\r\n", "\n");
    let entries = input.split("\n\n");

    entries
        .map(|e| e.parse::<Passport>().unwrap())
        .filter(|p| p.validate_keys())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn part1_example() {
        assert_eq!(2, part1(EXAMPLE.to_string()));
    }
}
