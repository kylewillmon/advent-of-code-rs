use std::convert::Infallible;

pub fn part1(input: String) -> Result<u32, Infallible>
{
    let total = parse_input(input)
        .into_iter()
        .map(|x| calc_fuel(x))
        .sum();
    Ok(total)
}

pub fn part2(input: String) -> Result<u32, Infallible>
{
    let total = parse_input(input)
        .into_iter()
        .map(|x| calc_fuel_recurse(x))
        .sum();
    Ok(total)
}

fn calc_fuel(weight: u32) -> u32 {
    if weight <= 6 {
        return 0
    }
    weight/3 - 2
}

fn calc_fuel_recurse(weight: u32) -> u32 {
    let fuel = calc_fuel(weight);
    if fuel == 0 {
        return 0
    }
    fuel + calc_fuel_recurse(fuel)
}

fn parse_input(input: String) -> Vec<u32>
{
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}
