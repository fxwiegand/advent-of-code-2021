use itertools::Itertools;

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
    let mut fishes = input
        .split(',')
        .map(|s| Lanternfish::from_str(s))
        .collect_vec();
    for _ in 1..=256 {
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
