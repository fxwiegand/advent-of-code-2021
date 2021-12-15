use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::prelude::NodeIndex;
use petgraph::{prelude::*, Graph};
use std::collections::HashMap;

pub(crate) fn solve_day15() -> u32 {
    let input = include_str!("../puzzles/day15.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut graph = Graph::<String, u32>::new();

    let mut map = HashMap::new();

    let mut weight_map: HashMap<(NodeIndex, NodeIndex), u32> = HashMap::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, _point) in line.iter().enumerate() {
            let name = format!("{} {}", y.to_string(), x.to_string());
            let p = graph.add_node(name);
            map.insert((y, x), p);
        }
    }

    for (y, line) in grid.iter().enumerate() {
        for (x, _point) in line.iter().enumerate() {
            if x < grid.len() - 1 {
                graph.add_edge(
                    *map.get(&(y, x)).unwrap(),
                    *map.get(&(y, x + 1)).unwrap(),
                    grid[y][x + 1],
                );
                weight_map.insert(
                    (*map.get(&(y, x)).unwrap(), *map.get(&(y, x + 1)).unwrap()),
                    grid[y][x + 1],
                );
                graph.add_edge(
                    *map.get(&(y, x + 1)).unwrap(),
                    *map.get(&(y, x)).unwrap(),
                    grid[y][x],
                );
                weight_map.insert(
                    (*map.get(&(y, x + 1)).unwrap(), *map.get(&(y, x)).unwrap()),
                    grid[y][x],
                );
            }
            if y < grid.len() - 1 {
                graph.add_edge(
                    *map.get(&(y, x)).unwrap(),
                    *map.get(&(y + 1, x)).unwrap(),
                    grid[y + 1][x],
                );
                weight_map.insert(
                    (*map.get(&(y, x)).unwrap(), *map.get(&(y + 1, x)).unwrap()),
                    grid[y + 1][x],
                );
                graph.add_edge(
                    *map.get(&(y + 1, x)).unwrap(),
                    *map.get(&(y, x)).unwrap(),
                    grid[y][x],
                );
                weight_map.insert(
                    (*map.get(&(y + 1, x)).unwrap(), *map.get(&(y, x)).unwrap()),
                    grid[y][x],
                );
            }
        }
    }

    let inf = std::u32::MAX;

    let start = map.get(&(0, 0)).unwrap();
    let target = map.get(&(grid.len() - 1, grid.len() - 1)).unwrap();

    let res = dijkstra(&graph, *start, Some(*target), |edge| {
        if let Some(weight) = weight_map.get(&(edge.source(), edge.target())) {
            *weight
        } else {
            inf
        }
    });

    *res.get(target).unwrap()
}

pub(crate) fn solve_day15_part2() -> u32 {
    let input = include_str!("../puzzles/day15.txt");
    let mut grid: Vec<Vec<u32>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut new_grid = grid.clone();
    for (y, line) in grid.iter().enumerate() {
        for i in 1..5 {
            new_grid[y].extend(
                line.iter()
                    .map(|x| if (x + i) > 9 { (x + i) % 10 + 1 } else { x + i }),
            );
        }
    }

    let temp_grid = new_grid.clone();

    for i in 1..5 {
        for line in &temp_grid {
            new_grid.push(
                line.iter()
                    .map(|x| if (x + i) > 9 { (x + i) % 10 + 1 } else { x + i })
                    .collect_vec(),
            )
        }
    }

    grid = new_grid;

    let mut graph = Graph::<String, u32>::new();

    let mut map = HashMap::new();

    let mut weight_map: HashMap<(NodeIndex, NodeIndex), u32> = HashMap::new();

    for (y, line) in grid.iter().enumerate() {
        for (x, _point) in line.iter().enumerate() {
            let name = format!("{} {}", y.to_string(), x.to_string());
            let p = graph.add_node(name);
            map.insert((y, x), p);
        }
    }

    for (y, line) in grid.iter().enumerate() {
        for (x, _point) in line.iter().enumerate() {
            if x < grid.len() - 1 {
                graph.add_edge(
                    *map.get(&(y, x)).unwrap(),
                    *map.get(&(y, x + 1)).unwrap(),
                    grid[y][x + 1],
                );
                weight_map.insert(
                    (*map.get(&(y, x)).unwrap(), *map.get(&(y, x + 1)).unwrap()),
                    grid[y][x + 1],
                );
                graph.add_edge(
                    *map.get(&(y, x + 1)).unwrap(),
                    *map.get(&(y, x)).unwrap(),
                    grid[y][x],
                );
                weight_map.insert(
                    (*map.get(&(y, x + 1)).unwrap(), *map.get(&(y, x)).unwrap()),
                    grid[y][x],
                );
            }
            if y < grid.len() - 1 {
                graph.add_edge(
                    *map.get(&(y, x)).unwrap(),
                    *map.get(&(y + 1, x)).unwrap(),
                    grid[y + 1][x],
                );
                weight_map.insert(
                    (*map.get(&(y, x)).unwrap(), *map.get(&(y + 1, x)).unwrap()),
                    grid[y + 1][x],
                );
                graph.add_edge(
                    *map.get(&(y + 1, x)).unwrap(),
                    *map.get(&(y, x)).unwrap(),
                    grid[y][x],
                );
                weight_map.insert(
                    (*map.get(&(y + 1, x)).unwrap(), *map.get(&(y, x)).unwrap()),
                    grid[y][x],
                );
            }
        }
    }

    let inf = std::u32::MAX;

    let start = map.get(&(0, 0)).unwrap();
    let target = map.get(&(grid.len() - 1, grid.len() - 1)).unwrap();

    let res = dijkstra(&graph, *start, Some(*target), |edge| {
        if let Some(weight) = weight_map.get(&(edge.source(), edge.target())) {
            *weight
        } else {
            inf
        }
    });

    *res.get(target).unwrap()
}
