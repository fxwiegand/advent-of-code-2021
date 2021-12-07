use itertools::Itertools;
use std::str::FromStr;

const SEARCH_RANGE: i32 = 500;

pub(crate) fn solve_day7() -> i32 {
    let input = include_str!("../puzzles/day7.txt");
    let crabs = input
        .split(',')
        .map(|s| i32::from_str(s).unwrap())
        .collect_vec();
    let mean = mean(&crabs);
    let mut solutions = Vec::new();
    for position in mean - SEARCH_RANGE..mean + SEARCH_RANGE {
        let fuel: i32 = crabs.iter().map(|c| (c - position).abs()).sum();
        solutions.push(fuel);
    }
    *solutions.iter().min().unwrap()
}

pub(crate) fn solve_day7_part2() -> i32 {
    let input = include_str!("../puzzles/day7.txt");
    let crabs = input
        .split(',')
        .map(|s| i32::from_str(s).unwrap())
        .collect_vec();
    let mean = mean(&crabs);
    let mut solutions = Vec::new();
    for position in mean - SEARCH_RANGE..mean + SEARCH_RANGE {
        let fuel: i32 = crabs
            .iter()
            .map(|c| (c - position).abs())
            .map(|c| (c * (c + 1)) / 2)
            .sum();
        solutions.push(fuel);
    }
    *solutions.iter().min().unwrap()
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().sum();
    sum / numbers.len() as i32
}
