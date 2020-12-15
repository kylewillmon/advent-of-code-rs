use std::io::{self, Read};
use std::fs;
use clap::{App, Arg};
use aoclib::{self, AOC, Day};

pub(crate) mod parse;
pub(crate) mod error;
mod day1;
mod day2;
mod day3;
mod day4;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {
    let m = App::new("Advent of Code 2020 solvers")
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
            .part(1, day1::part1)
            .part(2, day1::part2))
        .day(Day::new(2)
            .part(1, day2::part1)
            .part(2, day2::part2))
        .day(Day::new(3)
            .part(1, day3::part1)
            .part(2, day3::part2))
        .day(Day::new(4)
            .part(1, day4::part1)
            .part(2, day4::part2))
        .day(Day::new(6)
            .part(1, day6::part1)
            .part(2, day6::part2))
        .day(Day::new(7)
            .part(1, day7::part1)
            .part(2, day7::part2))
        .day(Day::new(8)
            .part(1, day8::part1)
            .part(2, day8::part2))
        .day(Day::new(9)
            .part(1, day9::part1)
            .part(2, day9::part2))
        .day(Day::new(10)
            .part(1, day10::part1)
            .part(2, day10::part2))
        .day(Day::new(11)
            .part(1, day11::part1)
            .part(2, day11::part2))
        .day(Day::new(12)
            .part(1, day12::part1)
            .part(2, day12::part2))
        .day(Day::new(13)
            .part(1, day13::part1)
            .part(2, day13::part2))
        .day(Day::new(14)
            .part(1, day14::part1)
            .part(2, day14::part2))
        .day(Day::new(15)
            .part(1, day15::part1)
            .part(2, day15::part2));

    match get_input(m.value_of("INPUT").unwrap()) {
        Ok(input) => print!("{}", aoc.run(day, input)),
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
