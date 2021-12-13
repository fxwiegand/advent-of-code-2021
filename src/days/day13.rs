use itertools::Itertools;
use std::str::FromStr;

pub(crate) fn solve_day13() -> usize {
    let input = include_str!("../puzzles/day13.txt");
    let (points, instructions) = input.split_once("\n\n").unwrap();
    let points: Vec<(u32, u32)> = points
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect_vec();
    let x_max = points.iter().map(|(x, y)| x).max().unwrap();
    let y_max = points.iter().map(|(x, y)| y).max().unwrap();
    let mut sheet = vec![vec![false; *x_max as usize + 1]; *y_max as usize + 1];
    for (x, y) in points {
        sheet[y as usize][x as usize] = true;
    }

    let mut sheets = Vec::new();
    sheets.push(sheet);

    for line in instructions.lines().take(1) {
        let coordinates = line.split_whitespace().collect_vec().pop().unwrap();
        let (fold_direction, fold_line) = coordinates.split_once('=').unwrap();
        let new_sheet = fold_sheet(
            sheets.pop().unwrap(),
            fold_direction == "y",
            usize::from_str(fold_line).unwrap(),
        );
        sheets.push(new_sheet);
    }
    let result = sheets.pop().unwrap();
    result.iter().flatten().filter(|b| **b).count()
}

fn fold_sheet(sheet: Vec<Vec<bool>>, up: bool, folding_line: usize) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    if up {
        // Fold up
        for (a, b) in sheet
            .iter()
            .take(folding_line)
            .zip_eq(sheet.iter().skip(folding_line + 1).rev())
        {
            let mut new_line = Vec::new();
            for (c, d) in a.iter().zip_eq(b.iter()) {
                new_line.push(*c || *d);
            }
            result.push(new_line);
        }
    } else {
        // Fold left
        for line in sheet {
            let mut new_line = Vec::new();
            for (a, b) in line
                .iter()
                .take(folding_line)
                .zip_eq(line.iter().skip(folding_line + 1).rev())
            {
                new_line.push(*a || *b);
            }
            result.push(new_line);
        }
    }
    result
}

pub(crate) fn solve_day13_part2() -> String {
    let input = include_str!("../puzzles/day13.txt");
    let (points, instructions) = input.split_once("\n\n").unwrap();
    let points: Vec<(u32, u32)> = points
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect_vec();
    let x_max = points.iter().map(|(x, y)| x).max().unwrap();
    let y_max = points.iter().map(|(x, y)| y).max().unwrap();
    let mut sheet = vec![vec![false; *x_max as usize + 1]; *y_max as usize + 1];
    for (x, y) in points {
        sheet[y as usize][x as usize] = true;
    }

    let mut sheets = Vec::new();
    sheets.push(sheet);

    for line in instructions.lines() {
        let coordinates = line.split_whitespace().collect_vec().pop().unwrap();
        let (fold_direction, fold_line) = coordinates.split_once('=').unwrap();
        let new_sheet = fold_sheet(
            sheets.pop().unwrap(),
            fold_direction == "y",
            usize::from_str(fold_line).unwrap(),
        );
        sheets.push(new_sheet);
    }
    let result = sheets.pop().unwrap();
    let r: Vec<String> = result
        .iter()
        .map(|x| x.iter().map(|b| if *b { '#' } else { ' ' }).collect())
        .collect_vec();
    String::from("\n") + &r.iter().join("\n")
}
