use itertools::Itertools;
use std::str::FromStr;

#[derive(Default, Debug)]
struct Position {
    horizontal_position: i32,
    vertical_position: i32,
}

impl Position {
    fn move_horizontally(&mut self, distance: i32) {
        self.horizontal_position += distance;
    }

    fn move_vertically(&mut self, depth: i32) {
        self.vertical_position += depth;
    }

    fn product_coordinates(&self) -> i32 {
        self.vertical_position * self.horizontal_position
    }
}

pub(crate) fn solve_day2() -> i32 {
    let input = include_str!("../puzzles/day2.txt");
    let mut position = Position::default();

    for line in input.lines() {
        let command = line.split_whitespace().take(1).collect_vec().pop().unwrap();
        let units = i32::from_str(
            line.split_whitespace()
                .skip(1)
                .take(1)
                .collect_vec()
                .pop()
                .unwrap(),
        )
        .unwrap();
        match command {
            "forward" => position.move_horizontally(units),
            "down" => position.move_vertically(units),
            "up" => position.move_vertically(-units),
            _ => unreachable!(),
        }
    }
    position.product_coordinates()
}

#[derive(Default, Debug)]
struct UpdgradedPosition {
    horizontal_position: i32,
    vertical_position: i32,
    aim: i32,
}

impl UpdgradedPosition {
    fn update_aim(&mut self, aim: i32) {
        self.aim += aim;
    }

    fn move_submarine(&mut self, distance: i32) {
        self.horizontal_position += distance;
        self.vertical_position += self.aim * distance;
    }

    fn product_coordinates(&self) -> i32 {
        self.vertical_position * self.horizontal_position
    }
}

pub(crate) fn solve_day2_part2() -> i32 {
    let input = include_str!("../puzzles/day2.txt");
    let mut position = UpdgradedPosition::default();

    for line in input.lines() {
        let command = line.split_whitespace().take(1).collect_vec().pop().unwrap();
        let units = i32::from_str(
            line.split_whitespace()
                .skip(1)
                .take(1)
                .collect_vec()
                .pop()
                .unwrap(),
        )
        .unwrap();
        match command {
            "forward" => position.move_submarine(units),
            "down" => position.update_aim(units),
            "up" => position.update_aim(-units),
            _ => unreachable!(),
        }
    }
    position.product_coordinates()
}
