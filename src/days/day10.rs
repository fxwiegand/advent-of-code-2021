use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

pub(crate) fn solve_day10() -> u32 {
    let input = include_str!("../puzzles/day10.txt");
    input
        .lines()
        .map(|s| s.chars().collect_vec())
        .map(|v| get_corruption_score(v))
        .sum()
}

pub(crate) fn solve_day10_part2() -> u64 {
    let input = include_str!("../puzzles/day10.txt");
    let mut scores = input
        .lines()
        .map(|s| s.chars().collect_vec())
        .filter(|v| get_corruption_score(v.to_vec()) == 0)
        .map(|v| get_incomplete_score(v))
        .collect_vec();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn get_corruption_score(line: Vec<char>) -> u32 {
    let mut stack = Vec::new();
    let mut scores = HashMap::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('>', 25137);
    scores.insert('}', 1197);

    for c in line {
        match c {
            '(' => stack.push('('),
            '{' => stack.push('{'),
            '<' => stack.push('<'),
            '[' => stack.push('['),
            ')' => {
                if let Some(d) = stack.pop() {
                    if d != '(' {
                        return *scores.get(&')').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            '}' => {
                if let Some(d) = stack.pop() {
                    if d != '{' {
                        return *scores.get(&'}').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            '>' => {
                if let Some(d) = stack.pop() {
                    if d != '<' {
                        return *scores.get(&'>').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            ']' => {
                if let Some(d) = stack.pop() {
                    if d != '[' {
                        return *scores.get(&']').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
    0
}

fn get_incomplete_score(line: Vec<char>) -> u64 {
    let mut stack = Vec::new();
    let mut scores = HashMap::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('>', 25137);
    scores.insert('}', 1197);

    for c in line {
        match c {
            '(' => stack.push('('),
            '{' => stack.push('{'),
            '<' => stack.push('<'),
            '[' => stack.push('['),
            ')' => {
                if let Some(d) = stack.pop() {
                    if d != '(' {
                        return *scores.get(&')').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            '}' => {
                if let Some(d) = stack.pop() {
                    if d != '{' {
                        return *scores.get(&'}').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            '>' => {
                if let Some(d) = stack.pop() {
                    if d != '<' {
                        return *scores.get(&'>').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            ']' => {
                if let Some(d) = stack.pop() {
                    if d != '[' {
                        return *scores.get(&']').unwrap();
                    }
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
    let mut p = 0;

    let mut other_scores = HashMap::new();
    other_scores.insert('(', 1);
    other_scores.insert('[', 2);
    other_scores.insert('<', 4);
    other_scores.insert('{', 3);

    for s in stack.iter().rev() {
        p = p * 5;
        p += other_scores.get(s).unwrap();
    }
    p
}
