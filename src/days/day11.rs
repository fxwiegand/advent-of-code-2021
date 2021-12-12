use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day11() -> u32 {
    let input = include_str!("../puzzles/day11.txt");
    let mut octopuses: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut flashes = 0;

    for _ in 1..=100 {
        let mut has_flashed = vec![vec![false; 10]; 10];
        for line in octopuses.iter_mut() {
            for octopus in line.iter_mut() {
                *octopus += 1;
            }
        }

        let mut someone_did_not_flash = false;

        while !someone_did_not_flash {
            let mut someone_flashed = false;
            let mut update = Vec::new();
            for (y, line) in octopuses.iter().enumerate() {
                for (x, octopus) in line.iter().enumerate() {
                    if octopus > &9 && !has_flashed[y][x] {
                        someone_flashed = true;
                        has_flashed[y][x] = true;
                        let tuples = get_adjacent_octopuses(y as i32, x as i32, 10);
                        update.extend(tuples);
                    }
                }
            }
            for (y, x) in update {
                octopuses[y][x] += 1;
            }
            someone_did_not_flash = !someone_flashed;
        }

        for (y, line) in has_flashed.iter().enumerate() {
            for (x, b) in line.iter().enumerate() {
                if *b {
                    octopuses[y][x] = 0;
                    flashes += 1;
                }
            }
        }
    }
    flashes
}

fn get_adjacent_octopuses(y: i32, x: i32, size: i32) -> Vec<(usize, usize)> {
    let adjacents = vec![
        (y - 1, x),
        (y + 1, x),
        (y - 1, x - 1),
        (y, x - 1),
        (y + 1, x - 1),
        (y - 1, x + 1),
        (y, x + 1),
        (y + 1, x + 1),
    ];
    adjacents
        .iter()
        .filter(|(a, b)| a >= &0 && a < &size && b >= &0 && b < &size)
        .map(|(a, b)| (*a as usize, *b as usize))
        .collect()
}

pub(crate) fn solve_day11_part2() -> u32 {
    let input = include_str!("../puzzles/day11.txt");
    let mut octopuses: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut step = 1;

    loop {
        let mut has_flashed = vec![vec![false; 10]; 10];
        for line in octopuses.iter_mut() {
            for octopus in line.iter_mut() {
                *octopus += 1;
            }
        }

        let mut someone_did_not_flash = false;

        while !someone_did_not_flash {
            let mut someone_flashed = false;
            let mut update = Vec::new();
            for (y, line) in octopuses.iter().enumerate() {
                for (x, octopus) in line.iter().enumerate() {
                    if octopus > &9 && !has_flashed[y][x] {
                        someone_flashed = true;
                        has_flashed[y][x] = true;
                        let tuples = get_adjacent_octopuses(y as i32, x as i32, 10);
                        update.extend(tuples);
                    }
                }
            }
            for (y, x) in update {
                octopuses[y][x] += 1;
            }
            someone_did_not_flash = !someone_flashed;
        }

        for (y, line) in has_flashed.iter().enumerate() {
            for (x, b) in line.iter().enumerate() {
                if *b {
                    octopuses[y][x] = 0;
                }
            }
        }

        if has_flashed.iter().flatten().filter(|b| **b).count() == 100 {
            break;
        }
        step += 1;
    }
    step
}
