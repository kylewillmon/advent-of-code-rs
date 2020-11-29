use std::io::{self, Read};
use std::fs;
use clap::{App, Arg};
use aoclib::{self, AOC, Day};

fn main() {
    let m = App::new("Advent of Code 2019 solvers")
                    .author("Kyle Willmon <kylewillmon@gmail.com>")
                    .arg(Arg::from_usage("<INPUT> 'Sets the input file to use'"))
                    .arg(Arg::with_name("day")
                        .short("d")
                        .long("day")
                        .takes_value(true)
                        .help("day to solve"))
                    .get_matches();

    let day = match m.value_of("day") {
        None => None,
        Some(val) =>
            match val.parse::<u8>() {
                Ok(val) => Some(val),
                Err(err) => {
                    println!("Invalid day {:?}: {}", val, err);
                    return;
                }
            }
    };

    let aoc = AOC::new()
        .day(Day::new(1)
            .part(1, |_| 1));

    match get_input(m.value_of("INPUT").unwrap()) {
        Ok(input) => println!("{}", aoc.run(day, input)),
        Err(err) => println!("Error: {}", err),
    };
}

fn get_input<P: AsRef<str>>(filename: P) -> io::Result<String>
{
    if filename.as_ref() == "-" {
        let mut data = String::new();
        return io::stdin().read_to_string(&mut data).map(move |_| data);
    }
    fs::read_to_string(filename.as_ref())
}
