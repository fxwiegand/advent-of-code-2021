mod days;

use days::day1;
use days::day10;
use days::day11;
use days::day12;
use days::day13;
use days::day2;
use days::day3;
use days::day4;
use days::day5;
use days::day6;
use days::day7;
use days::day8;
use days::day9;
use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::from_args();

    let result = match (opts.day, opts.part_two) {
        (1, false) => day1::solve_day1().to_string(),
        (1, true) => day1::solve_day1_part2().to_string(),
        (2, false) => day2::solve_day2().to_string(),
        (2, true) => day2::solve_day2_part2().to_string(),
        (3, false) => day3::solve_day3().to_string(),
        (3, true) => day3::solve_day3_part2().to_string(),
        (4, false) => day4::solve_day4().to_string(),
        (4, true) => day4::solve_day4_part2().to_string(),
        (5, false) => day5::solve_day5().to_string(),
        (5, true) => day5::solve_day5_part2().to_string(),
        (6, false) => day6::solve_day6().to_string(),
        (6, true) => day6::solve_day6_part2().to_string(),
        (7, false) => day7::solve_day7().to_string(),
        (7, true) => day7::solve_day7_part2().to_string(),
        (8, false) => day8::solve_day8().to_string(),
        (8, true) => day8::solve_day8_part2().to_string(),
        (9, false) => day9::solve_day9().to_string(),
        (9, true) => day9::solve_day9_part2().to_string(),
        (10, false) => day10::solve_day10().to_string(),
        (10, true) => day10::solve_day10_part2().to_string(),
        (11, false) => day11::solve_day11().to_string(),
        (11, true) => day11::solve_day11_part2().to_string(),
        (12, false) => day12::solve_day12().to_string(),
        (12, true) => day12::solve_day12_part2().to_string(),
        (13, false) => day13::solve_day13().to_string(),
        (13, true) => day13::solve_day13_part2().to_string(),
        _ => unimplemented!(),
    };

    if opts.part_two {
        println!(
            "The solution for the second part of day {} is {}.",
            opts.day, result
        );
    } else {
        println!("The solution for day {} is {}.", opts.day, result);
    }

    Ok(())
}

#[derive(StructOpt, Debug)]
struct Opts {
    /// The day of advent of code.
    day: usize,
    /// Whether you want the solution for part 1 or 2.
    #[structopt(short, long)]
    part_two: bool,
}
