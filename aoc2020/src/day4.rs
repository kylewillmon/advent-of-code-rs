use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

struct Passport(HashMap<String, String>);

fn is_num_between<T: AsRef<str>>(val: T, min: u32, max: u32) -> bool {
    val.as_ref()
        .parse::<u32>()
        .map(|num| min <= num && num <= max)
        .unwrap_or(false)
}

impl Passport {
    fn validate_keys(&self) -> bool {
        let required_keys: HashSet<String> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .map(|s| s.to_string())
            .collect();

        let keys: HashSet<String> = self.0.keys().cloned().collect();

        keys.is_superset(&required_keys)
    }

    fn validate_num<T: AsRef<str>>(&self, key: T, min: u32, max: u32) -> bool {
        self.0
            .get(&String::from(key.as_ref()))
            .map(|val| is_num_between(val, min, max))
            .unwrap_or(false)
    }

    fn validate_height(&self) -> bool {
        self.0
            .get(&"hgt".to_string())
            .map(|val| {
                if let Some(val) = val.strip_suffix("in") {
                    return is_num_between(val, 59, 76);
                }
                if let Some(val) = val.strip_suffix("cm") {
                    return is_num_between(val, 150, 193);
                }
                false
            })
            .unwrap_or(false)
    }

    fn validate_hair_color(&self) -> bool {
        self.0
            .get(&"hcl".to_string())
            .map(|val| {
                val.strip_prefix('#')
                    .map(|hex| {
                        hex.chars()
                            .all(|c| ('0'..='9').contains(&c) || ('a'..='f').contains(&c))
                    })
                    .unwrap_or(false)
            })
            .unwrap_or(false)
    }

    fn validate_eye_color(&self) -> bool {
        self.0
            .get(&"ecl".to_string())
            .map(|val| {
                matches!(
                    val.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                )
            })
            .unwrap_or(false)
    }

    fn validate_id(&self) -> bool {
        self.0
            .get(&"pid".to_string())
            .map(|val| val.len() == 9 && val.chars().all(|c| c.is_ascii_digit()))
            .unwrap_or(false)
    }

    fn validate(&self) -> bool {
        self.validate_keys()
            && self.validate_num("byr", 1920, 2002)
            && self.validate_num("iyr", 2010, 2020)
            && self.validate_num("eyr", 2020, 2030)
            && self.validate_height()
            && self.validate_hair_color()
            && self.validate_eye_color()
            && self.validate_id()
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

pub fn part1(input: String) -> Result<usize, Infallible> {
    let entries = input.split("\n\n");

    let num = entries
        .map(|e| e.parse::<Passport>().unwrap())
        .filter(|p| p.validate_keys())
        .count();
    Ok(num)
}

pub fn part2(input: String) -> Result<usize, Infallible> {
    let entries = input.split("\n\n");

    let num = entries
        .map(|e| e.parse::<Passport>().unwrap())
        .filter(|p| p.validate())
        .count();
    Ok(num)
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
        assert_eq!(Ok(2), part1(EXAMPLE.to_string()));
    }

    const INVALID: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    #[test]
    fn part2_invalid() {
        assert_eq!(Ok(0), part2(INVALID.to_string()));
    }

    const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn part2_valid() {
        assert_eq!(Ok(4), part2(VALID.to_string()));
    }
}
