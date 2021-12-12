use std::collections::HashMap;

pub(crate) fn solve_day12() -> u32 {
    let input = include_str!("../puzzles/day12.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        let (cave, cave2) = line.split_once('-').unwrap();
        let entry = map.entry(cave.to_owned()).or_insert_with(Vec::new);
        entry.push(cave2.to_string());
        let entry2 = map.entry(cave2.to_owned()).or_insert_with(Vec::new);
        entry2.push(cave.to_string());
    }
    get_paths("start".to_string(), &map, vec!["start".to_string()])
}

fn get_paths(cave: String, map: &HashMap<String, Vec<String>>, visited: Vec<String>) -> u32 {
    if cave == "end" {
        return 1;
    }
    let mut sum = 0;
    for neighbour in map.get(&cave).unwrap() {
        if !(visited.contains(neighbour) && &neighbour.to_lowercase() == neighbour) {
            let mut seen = visited.clone();
            seen.push(neighbour.to_string());
            sum += get_paths(neighbour.to_string(), &map.clone(), seen)
        }
    }
    sum
}

fn get_paths2(
    cave: String,
    map: &HashMap<String, Vec<String>>,
    visited: Vec<String>,
    twice: bool,
) -> u32 {
    if cave == "end" {
        return 1;
    }
    let mut sum = 0;
    for neighbour in map.get(&cave).unwrap() {
        if neighbour != "start" {
            if !visited.contains(neighbour) || &neighbour.to_lowercase() != neighbour {
                let mut seen = visited.clone();
                seen.push(neighbour.to_string());
                sum += get_paths2(neighbour.to_string(), &map.clone(), seen.clone(), twice);
            } else if !twice {
                let mut seen = visited.clone();
                seen.push(neighbour.to_string());
                sum += get_paths2(neighbour.to_string(), &map.clone(), seen.clone(), true);
            }
        }
    }
    sum
}

pub(crate) fn solve_day12_part2() -> u32 {
    let input = include_str!("../puzzles/day12.txt");
    let mut map = HashMap::new();
    for line in input.lines() {
        let (cave, cave2) = line.split_once('-').unwrap();
        let entry = map.entry(cave.to_owned()).or_insert_with(Vec::new);
        entry.push(cave2.to_string());
        let entry2 = map.entry(cave2.to_owned()).or_insert_with(Vec::new);
        entry2.push(cave.to_string());
    }
    get_paths2("start".to_string(), &map, vec!["start".to_string()], false)
}
