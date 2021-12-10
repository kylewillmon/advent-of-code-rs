use std::collections::HashMap;
use std::iter::repeat;

use super::error::AocError;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct BagIdx(usize);

struct Bag {
    name: String,
    must_contain: Vec<(u32, BagIdx)>,
}

struct BagGraph {
    bags: Vec<Bag>,
}

impl BagGraph {
    fn new() -> Self {
        BagGraph {
            bags: Vec::new(),
        }
    }

    fn add_bag(&mut self, name: String) -> BagIdx {
        self.bags.push(Bag{
            name,
            must_contain: Vec::new(),
        });
        BagIdx(self.bags.len() - 1)
    }

    fn must_contain(&mut self, outer: BagIdx, inner: BagIdx, count: u32) {
        self.bags[outer.0].must_contain.push((count, inner));
    }

    fn find_bag(&self, name: String) -> Option<BagIdx> {
        self.bags
            .iter()
            .enumerate().find(|(_, bag)| bag.name == name)
            .map(|(i, _)| BagIdx(i))
    }

    fn can_contain(&self, bag: BagIdx) -> Vec<BagIdx> {
        let mut all = Vec::new();
        let mut cur = vec![bag];

        loop {
            let mut next = Vec::new();

            for (idx, b) in self.bags.iter().enumerate() {
                for inner in b.must_contain.iter() {
                    let contains = cur.iter().any(|c| *c == inner.1);
                    if contains {
                        next.push(BagIdx(idx))
                    }
                }
            }

            if next.is_empty() {
                break
            }

            all.extend(next.iter());
            cur.clear();
            cur.append(&mut next);
        }
        all.sort();
        all.dedup();
        all
    }

    fn calc_cost(&self, bag: BagIdx) -> u32 {
        let mut costs: Vec<Option<u32>> = repeat(None).take(self.bags.len()).collect();

        for (idx, b) in self.bags.iter().enumerate() {
            if b.must_contain.is_empty() {
                costs[idx] = Some(0);
            }
        }

        loop {
            if let Some(count) = costs[bag.0] {
                return count;
            }
            for (idx, b) in self.bags.iter().enumerate() {
                let inside: Option<Vec<u32>> = b.must_contain
                    .iter()
                    .map(|(c, i)| {
                        costs[i.0].map(|inner_cost| c * (inner_cost+1))
                    }).collect();

                if let Some(counts) = inside {
                    costs[idx] = Some(counts.iter().cloned().sum());
                }
            }
        }
    }
}

fn parse_input(input: String) -> Result<BagGraph, AocError> {
    let mut bags = BagGraph::new();
    let mut rules = Vec::new();
    let mut index: HashMap<String, BagIdx> = HashMap::new();

    for line in input.lines() {
        let mut split = line.splitn(2, " bags contain ");
        let bagname = split.next().unwrap();
        let rule = split.next().ok_or(AocError::ParseError("invalid line".to_string()))?;

        let idx = bags.add_bag(bagname.to_string());
        index.insert(bagname.to_string(), idx);
        rules.push((idx, rule));
    }

    for (idx, rule) in rules {
        let contains = rule
            .trim_end_matches('.')
            .split(", ")
            .map(|s| {
                s.trim_end_matches('s')
                    .strip_suffix(" bag")
                    .ok_or(AocError::ParseError("invalid contain".to_string()))
            }).collect::<Result<Vec<&str>, AocError>>()?;

        for contain in contains {
            if contain == "no other" {
                continue;
            }
            let mut split = contain.splitn(2, ' ');
            let count = split.next().unwrap().parse::<u32>()?;
            let inner_name = split.next().ok_or(AocError::ParseError("no inner bag".to_string()))?;
            let inner_idx = index.get(inner_name).ok_or(AocError::ParseError("unknown contained bag".to_string()))?;

            bags.must_contain(idx, *inner_idx, count);
        }
    }
    Ok(bags)
}

pub fn part1(input: String) -> Result<usize, AocError> {
    let bags = parse_input(input)?;
    Ok(bags.can_contain(bags.find_bag("shiny gold".to_string()).unwrap()).len())
}

pub fn part2(input: String) -> Result<u32, AocError> {
    let bags = parse_input(input)?;
    Ok(bags.calc_cost(bags.find_bag("shiny gold".to_string()).unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn part1_example() {
        assert_eq!(Ok(4), part1(EXAMPLE.to_string()));
    }

    #[test]
    fn part2_example() {
        assert_eq!(Ok(32), part2(EXAMPLE.to_string()));
    }

    const EXAMPLE2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn part2_example2() {
        assert_eq!(Ok(126), part2(EXAMPLE2.to_string()));
    }
}