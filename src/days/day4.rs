use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Board {
    lines: Vec<HashSet<u32>>,
    lines_as_vec: Vec<Vec<u32>>,
}

impl Board {
    fn from_str(board: &str) -> Board {
        let mut temp = Vec::new();
        let mut lines = Vec::new();
        let mut lines_as_vec = Vec::new();
        for line in board.lines() {
            temp.push(
                line.split_whitespace()
                    .map(|s| u32::from_str(s).unwrap())
                    .collect_vec(),
            );
        }
        for row in &temp {
            let set: HashSet<_> = row.iter().copied().collect();
            lines_as_vec.push(row.to_vec());
            lines.push(set);
        }
        for i in 0..5 {
            let mut set = HashSet::new();
            let mut v = Vec::new();
            for row in &temp {
                set.insert(row[i]);
                v.push(row[i]);
            }
            lines.push(set);
            lines_as_vec.push(v);
        }
        Board {
            lines,
            lines_as_vec,
        }
    }

    fn has_won(&self, draws: &HashSet<u32>) -> bool {
        for set in &self.lines {
            if set.is_subset(draws) {
                return true;
            }
        }
        false
    }

    fn get_unmarked_numbers_sum(&self, draws: &HashSet<u32>) -> u32 {
        let unmarked = self.lines_as_vec.clone();
        let s: u32 = unmarked
            .iter()
            .flatten()
            .filter(|i| !draws.contains(i))
            .copied()
            .sum();
        // Devide by 2 because all rows and columns occur twice
        s / 2
    }
}
pub(crate) fn solve_day4() -> u32 {
    let input = include_str!("../puzzles/day4.txt");
    let (drawings, cards) = input.split_once("\n\n").unwrap();
    let mut boards = Vec::new();
    for card in cards.split("\n\n") {
        boards.push(Board::from_str(card));
    }
    let mut drawn_numbers = HashSet::new();
    let drawings = drawings
        .split(',')
        .map(|u| u32::from_str(u).unwrap())
        .collect_vec();
    for drawn_number in drawings {
        drawn_numbers.insert(drawn_number);
        for board in &boards {
            if board.has_won(&drawn_numbers) {
                return board.get_unmarked_numbers_sum(&drawn_numbers) * drawn_number;
            }
        }
    }
    0
}

pub(crate) fn solve_day4_part2() -> u32 {
    let input = include_str!("../puzzles/day4.txt");
    let (drawings, cards) = input.split_once("\n\n").unwrap();
    let mut boards = Vec::new();
    for card in cards.split("\n\n") {
        boards.push(Board::from_str(card));
    }
    let total_boards = boards.len();
    let pos_boards = boards.clone();
    let mut drawn_numbers = HashSet::new();
    let drawings = drawings
        .split(',')
        .map(|u| u32::from_str(u).unwrap())
        .collect_vec();
    let mut winning_board = &Board {
        lines: vec![],
        lines_as_vec: vec![],
    };
    let mut last_board = false;
    for drawn_number in drawings {
        drawn_numbers.insert(drawn_number);
        let mut won_boards = Vec::new();
        for board in &boards {
            if board.has_won(&drawn_numbers) {
                won_boards.push(board);
            }
        }
        if total_boards - won_boards.len() == 1 && !last_board {
            let b = pos_boards
                .iter()
                .filter(|b| !won_boards.contains(b))
                .collect_vec()
                .pop()
                .unwrap();
            winning_board = b;
            last_board = true;
        } else if total_boards - won_boards.len() == 0 && last_board {
            return winning_board.get_unmarked_numbers_sum(&drawn_numbers) * drawn_number;
        }
    }
    0
}
