use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Lanternfish {
    timer: i32,
}

impl Lanternfish {
    fn from_str(s: &str) -> Lanternfish {
        Lanternfish {
            timer: s.parse().unwrap(),
        }
    }

    fn pass_day(&mut self) -> Option<Lanternfish> {
        if self.timer > 0 {
            self.timer -= 1;
            return None;
        } else {
            self.timer = 6;
        }
        Some(Lanternfish { timer: 8 })
    }
}

pub(crate) fn solve_day6() -> usize {
    let input = include_str!("../puzzles/day6.txt");
    let mut fishes = input
        .split(',')
        .map(|s| Lanternfish::from_str(s))
        .collect_vec();
    for _ in 1..=80 {
        let mut newborns = Vec::new();
        for fish in fishes.iter_mut() {
            if let Some(newborn) = fish.pass_day() {
                newborns.push(newborn);
            }
        }
        fishes.extend(newborns.iter())
    }
    fishes.iter().count()
}

pub(crate) fn solve_day6_part2() -> usize {
    let input = include_str!("../puzzles/day6.txt");
    let fishes = input
        .split(',')
        .map(|s| i32::from_str(s).unwrap())
        .collect_vec();
    let mut fish_map = HashMap::new();
    for fish in fishes {
        let counter = fish_map.entry(fish).or_insert(0);
        *counter += 1;
    }
    for _ in 1..=256 {
        let mut new_map = HashMap::new();
        for (timer, f) in fish_map.iter() {
            if timer > &0 {
                let counter = new_map.entry(timer - 1).or_insert(0);
                *counter += *f;
            } else {
                let counter = new_map.entry(6).or_insert(0);
                *counter += *f;
                new_map.insert(8, *f);
            }
        }
        fish_map = new_map;
    }
    fish_map.iter().map(|(_, v)| v).copied().sum()
}
