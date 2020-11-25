mod day;

pub use day::Day;

pub struct AOC {
    year: u16,
    days: Vec<Day>,
}

impl AOC {
    pub fn new(year: u16) -> Self {
        AOC {
            year,
            days: Vec::new(),
        }
    }

    pub fn day(&mut self, d: day::Day) -> &mut Self {
        self.days.push(d);
        self
    }

    pub fn run(&self) {
        println!("AOC year {}", self.year);
    }
}