mod day;
mod solver;
pub mod strtools;

pub use day::Day;

pub struct AOC<'a> {
    days: Vec<Day<'a>>,
}

impl<'a> AOC<'a> {
    pub fn new() -> Self {
        AOC { days: Vec::new() }
    }

    pub fn day<'b>(mut self, d: day::Day<'b>) -> Self
    where
        'b: 'a,
    {
        self.days.push(d);
        self
    }

    pub fn run(self, day: Option<u8>, input: String) -> String {
        let d = if let Some(day) = day {
            self.days.into_iter().find(|x| x.day == day)
        } else {
            self.days.into_iter().max_by_key(|x| x.day)
        };

        match d {
            Some(d) => d.solve(input),
            None => "Error: Day not found".to_string(),
        }
    }
}

impl<'a> Default for AOC<'a> {
    fn default() -> Self {
        Self::new()
    }
}
