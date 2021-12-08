use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

pub(crate) fn solve_day8() -> usize {
    let input = include_str!("../puzzles/day8.txt");
    input
        .lines()
        .map(|i| i.split('|').collect_vec().pop().unwrap())
        .map(|s| s.split_whitespace().collect_vec())
        .flatten()
        .filter(|s| s.len() <= 4 || s.len() == 7)
        .count()
}

pub(crate) fn solve_day8_part2() -> usize {
    let input = include_str!("../puzzles/day8.txt");
    input
        .lines()
        .map(|i| i.split_once('|').unwrap())
        .map(|(s, o)| {
            (
                s.split_whitespace().collect_vec(),
                o.split_whitespace().collect_vec(),
            )
        })
        .map(|(s, o)| solve_entry(s, o))
        .sum()
}

/// This is absolutely terrible
fn solve_entry(input: Vec<&str>, output: Vec<&str>) -> usize {
    let stmp = input
        .iter()
        .filter(|s| s.len() == 3)
        .map(|s| s.chars().collect_vec())
        .collect_vec()
        .pop()
        .unwrap();
    let seven: HashSet<char> = HashSet::from_iter(stmp.into_iter());
    let ftmp = input
        .iter()
        .filter(|s| s.len() == 4)
        .map(|s| s.chars().collect_vec())
        .collect_vec()
        .pop()
        .unwrap();
    let four: HashSet<char> = HashSet::from_iter(ftmp.into_iter());
    let toptmp: HashSet<char> = seven.symmetric_difference(&four).copied().collect();
    let top = toptmp.intersection(&seven).collect_vec().pop().unwrap();
    let six_segments_vec = input
        .iter()
        .filter(|s| s.len() == 6)
        .map(|s| s.chars().collect_vec())
        .collect_vec();
    let mut six_segments: Vec<HashSet<char>> = Vec::new();
    for s in six_segments_vec {
        six_segments.push(HashSet::from_iter(s.into_iter()));
    }
    let int: HashSet<char> = six_segments[0]
        .intersection(&six_segments[1])
        .copied()
        .collect();
    let full_int: HashSet<char> = int.intersection(&six_segments[2]).copied().collect();
    let ttmp = input
        .iter()
        .filter(|s| s.len() == 2)
        .map(|s| s.chars().collect_vec())
        .collect_vec()
        .pop()
        .unwrap();
    let two: HashSet<char> = HashSet::from_iter(ttmp.into_iter());
    let bottom_right = full_int.intersection(&two).collect_vec().pop().unwrap();
    let mut br = HashSet::new();
    br.insert(*bottom_right);
    let top_right = two.difference(&br).collect_vec().pop().unwrap();
    let find_bot: HashSet<char> = full_int.difference(&four).copied().collect();
    let mut t = HashSet::new();
    t.insert(*top);
    let bottom = find_bot.difference(&t).collect_vec().pop().unwrap();
    let get_m: HashSet<char> = four.difference(&full_int).copied().collect();
    let mut tr = HashSet::new();
    tr.insert(*top_right);
    let middle = get_m.difference(&tr).collect_vec().pop().unwrap();
    let mut trbrmi = HashSet::new();
    trbrmi.insert(*top_right);
    trbrmi.insert(*bottom_right);
    trbrmi.insert(*middle);
    let top_left = four.difference(&trbrmi).collect_vec().pop().unwrap();
    let tmp_eight = input
        .iter()
        .filter(|s| s.len() == 7)
        .map(|s| s.chars().collect_vec())
        .collect_vec()
        .pop()
        .unwrap();
    let eight: HashSet<char> = HashSet::from_iter(tmp_eight.iter().copied());
    let mut all_except_bl = HashSet::new();
    all_except_bl.insert(*top_right);
    all_except_bl.insert(*bottom_right);
    all_except_bl.insert(*middle);
    all_except_bl.insert(*top);
    all_except_bl.insert(*top_left);
    all_except_bl.insert(*bottom);
    let bottom_left = eight
        .difference(&all_except_bl)
        .collect_vec()
        .pop()
        .unwrap();

    let mut zero = vec![
        *top,
        *top_left,
        *top_right,
        *bottom_left,
        *bottom_right,
        *bottom,
    ];
    let mut one = vec![*top_right, *bottom_right];
    let mut two = vec![*top, *top_right, *middle, *bottom_left, *bottom];
    let mut three = vec![*top, *top_right, *middle, *bottom_right, *bottom];
    let mut four = vec![*top_left, *top_right, *middle, *bottom_right];
    let mut five = vec![*top, *top_left, *middle, *bottom_right, *bottom];
    let mut six = vec![
        *top,
        *top_left,
        *middle,
        *bottom_right,
        *bottom,
        *bottom_left,
    ];
    let mut seven = vec![*top, *top_right, *bottom_right];
    let mut eight = vec![
        *top,
        *top_left,
        *top_right,
        *bottom_left,
        *bottom_right,
        *bottom,
        *middle,
    ];
    let mut nine = vec![*top, *top_left, *top_right, *middle, *bottom_right, *bottom];

    zero.sort_unstable();
    one.sort_unstable();
    two.sort_unstable();
    three.sort_unstable();
    four.sort_unstable();
    five.sort_unstable();
    six.sort_unstable();
    seven.sort_unstable();
    eight.sort_unstable();
    nine.sort_unstable();

    let mut result = Vec::new();
    for o in output {
        let mut chars = o.chars().collect_vec();
        chars.sort_unstable();
        if chars == one {
            result.push('1')
        } else if chars == two {
            result.push('2')
        } else if chars == three {
            result.push('3')
        } else if chars == four {
            result.push('4')
        } else if chars == five {
            result.push('5')
        } else if chars == six {
            result.push('6')
        } else if chars == seven {
            result.push('7')
        } else if chars == eight {
            result.push('8')
        } else if chars == nine {
            result.push('9')
        } else {
            result.push('0')
        }
    }
    let str = String::from_iter(result.iter());
    usize::from_str(&str).unwrap()
}
