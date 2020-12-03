
struct Map(String);

impl Map {
    fn new(s: String) -> Self
    {
        Self(s)
    }

    fn count_trees(&self, dcol: usize, drow: usize) -> usize
    {
        let rows = self.0.lines().step_by(drow);
        let colnums = (0..).step_by(dcol);

        rows.zip(colnums)
            .map(|(row, colnum)| {
                let rowlen = row.chars().count();
                row.chars().nth(colnum % rowlen).unwrap()
            })
            .filter(|&c| c == '#')
            .count()
    }
}

pub fn part1(input: String) -> usize
{
    Map::new(input).count_trees(3, 1)
}


pub fn part2(input: String) -> usize
{
    let slopes = vec!(
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    );

    let m = Map::new(input);

    slopes.into_iter()
        .map(|(c, r)| m.count_trees(c, r))
        .fold(1, |a, b| a * b)
}
