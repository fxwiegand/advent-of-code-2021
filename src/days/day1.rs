use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day1() -> u32 {
    let input = include_str!("../puzzles/day1.txt");
    let mut increased = 0;
    let mut previous = 0;
    for line in input.lines() {
        let depth = u32::from_str(line.trim()).unwrap();
        if depth > previous {
            increased += 1;
        }
        previous = depth;
    }
    increased - 1
}

pub(crate) fn solve_day1_part2() -> u32 {
    let input = include_str!("../puzzles/day1.txt");
    let mut increased = 0;
    let mut previous = 0;
    for (line1, line2, line3) in input.lines().tuple_windows::<(_, _, _)>() {
        let depth1 = u32::from_str(line1.trim()).unwrap();
        let depth2 = u32::from_str(line2.trim()).unwrap();
        let depth3 = u32::from_str(line3.trim()).unwrap();
        let depth = depth1 + depth2 + depth3;
        if depth > previous {
            increased += 1;
        }
        previous = depth;
    }
    increased - 1
}
