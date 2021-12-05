mod days;

use days::day1;
use days::day2;
use days::day3;
use days::day4;
use days::day5;
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
