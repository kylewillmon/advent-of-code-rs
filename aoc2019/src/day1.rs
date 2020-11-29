
pub fn part1(input: String) -> u32
{
    parse_input(input)
        .into_iter()
        .map(|x| calc_fuel(x))
        .fold(0, |a, b| a+b)
}

pub fn part2(input: String) -> u32
{
    parse_input(input)
        .into_iter()
        .map(|x| calc_fuel_recurse(x))
        .fold(0, |a, b| a+b)
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
