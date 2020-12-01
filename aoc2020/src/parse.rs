// Useful input parsers

pub fn to_nums(input: String) -> Vec<u32>
{
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect()
}
