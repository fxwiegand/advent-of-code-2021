use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn solve_day12() -> u32 {
    let input = include_str!("../puzzles/day12.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        let (cave, connected) = line.split_once('-').unwrap();
        let entry = map.entry(cave.to_owned()).or_insert_with(Vec::new);
        entry.push(connected.to_string())
    }
    unimplemented!()
}

pub(crate) fn solve_day12_part2() -> u32 {
    let input = include_str!("../puzzles/day12.txt");
    unimplemented!()
}
