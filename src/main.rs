mod days;

use days::day1;
use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::from_args();

    let result = match (opts.day, opts.part_two) {
        (1, false) => day1::solve_day1().to_string(),
        (1, true) => day1::solve_day1_part2().to_string(),
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
