use itertools::Itertools;
use std::cmp::min;
use std::str::FromStr;

pub(crate) fn solve_day9() -> u32 {
    let input = include_str!("../puzzles/day9.txt");
    let map = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| u32::from_str(&c.to_string()).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            let mut lowest_neighbour = 10;
            if y as i32 > 0 {
                lowest_neighbour = min(map[y - 1][x], lowest_neighbour);
            }
            if y + 1 < map.len() {
                lowest_neighbour = min(map[y + 1][x], lowest_neighbour);
            }
            if x as i32 > 0 {
                lowest_neighbour = min(map[y][x - 1], lowest_neighbour);
            }
            if x + 1 < row.len() {
                lowest_neighbour = min(map[y][x + 1], lowest_neighbour);
            }
            if element < &lowest_neighbour {
                sum += element + 1;
            }
        }
    }
    sum
}

pub(crate) fn solve_day9_part2() -> usize {
    let input = include_str!("../puzzles/day9.txt");
    let map = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| u32::from_str(&c.to_string()).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut besin_position = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, element) in row.iter().enumerate() {
            let mut lowest_neighbour = 10;
            if y as i32 > 0 {
                lowest_neighbour = min(map[y - 1][x], lowest_neighbour);
            }
            if y + 1 < map.len() {
                lowest_neighbour = min(map[y + 1][x], lowest_neighbour);
            }
            if x as i32 > 0 {
                lowest_neighbour = min(map[y][x - 1], lowest_neighbour);
            }
            if x + 1 < row.len() {
                lowest_neighbour = min(map[y][x + 1], lowest_neighbour);
            }
            if element < &lowest_neighbour {
                besin_position.push((y, x));
            }
        }
    }
    let mut lengths = Vec::new();
    for (y, x) in besin_position {
        let mut v: Vec<(i32, i32)> = Vec::new();
        let mut besin_points = get_basin(y as i32, x as i32, &map.clone(), &mut v);
        besin_points.sort_unstable();
        besin_points.dedup();
        lengths.push(besin_points.len());
    }
    lengths.sort_unstable();
    lengths.iter().rev().take(3).product()
}

fn get_basin(y: i32, x: i32, map: &[Vec<u32>], visited: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    if x >= 0
        && x < map[0].len() as i32
        && y >= 0
        && y < map.len() as i32
        && !visited.contains(&(y, x))
        && map[y as usize][x as usize] < 9
    {
        result.push((y, x));
        visited.push((y, x));
        result.extend(get_basin(y, x - 1, map, visited));
        result.extend(get_basin(y, x + 1, map, visited));
        result.extend(get_basin(y - 1, x, map, visited));
        result.extend(get_basin(y + 1, x, map, visited));
    }
    result
}
