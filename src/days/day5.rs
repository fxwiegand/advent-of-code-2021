use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Line {
    fn from_str(input: &str) -> Line {
        let (start, end) = input.split_once(" -> ").unwrap();
        Line {
            start: Point::from_str(start),
            end: Point::from_str(end),
        }
    }

    fn is_non_diagonal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn is_diagonal(&self) -> bool {
        (self.start.x - self.end.x).abs() == (self.start.y - self.end.y).abs()
    }

    fn covers_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        match (self.is_non_diagonal(), self.start.x == self.end.x) {
            (true, true) => {
                if self.start.y < self.end.y {
                    for y in self.start.y..=self.end.y {
                        let p = Point { x: self.start.x, y };
                        points.push(p);
                    }
                } else {
                    for y in self.end.y..=self.start.y {
                        let p = Point { x: self.start.x, y };
                        points.push(p);
                    }
                }
            }
            (true, false) => {
                if self.start.x < self.end.x {
                    for x in self.start.x..=self.end.x {
                        let p = Point { x, y: self.start.y };
                        points.push(p);
                    }
                } else {
                    for x in self.end.x..=self.start.x {
                        let p = Point { x, y: self.start.y };
                        points.push(p);
                    }
                }
            }
            (false, _) => {
                if self.is_diagonal() {
                    let x_start = min(self.start.x, self.end.x);
                    let x_end = max(self.start.x, self.end.x);
                    let (y_start, y_end) = if x_start == self.start.x {
                        (self.start.y, self.end.y)
                    } else {
                        (self.end.y, self.start.y)
                    };
                    if y_start < y_end {
                        for (x, y) in (x_start..=x_end).zip(y_start..=y_end) {
                            points.push(Point { x, y })
                        }
                    } else {
                        for (x, y) in (x_start..=x_end).zip((y_end..=y_start).rev()) {
                            points.push(Point { x, y })
                        }
                    }
                }
            }
        }
        points
    }
}

impl Point {
    fn from_str(input: &str) -> Point {
        let (x, y) = input.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

pub(crate) fn solve_day5() -> usize {
    let input = include_str!("../puzzles/day5.txt");
    let covered_points = input
        .lines()
        .map(|l| Line::from_str(l))
        .filter(|l| l.is_non_diagonal())
        .map(|l| l.covers_points())
        .flatten()
        .collect_vec();
    let mut map = HashMap::new();
    for p in covered_points {
        let counter = map.entry(p).or_insert(0);
        *counter += 1;
    }
    map.iter().filter(|(_, count)| count > &&1).count()
}

pub(crate) fn solve_day5_part2() -> usize {
    let input = include_str!("../puzzles/day5.txt");
    let covered_points = input
        .lines()
        .map(|l| Line::from_str(l))
        .filter(|l| l.is_non_diagonal() || l.is_diagonal())
        .map(|l| l.covers_points())
        .flatten()
        .collect_vec();
    let mut map = HashMap::new();
    for p in covered_points {
        let counter = map.entry(p).or_insert(0);
        *counter += 1;
    }
    map.iter().filter(|(_, count)| count > &&1).count()
}
