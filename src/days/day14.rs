use itertools::Itertools;
use std::collections::HashMap;

pub(crate) fn solve_day14() -> i32 {
    let input = include_str!("../puzzles/day14.txt");
    let (template, unparsed_rules) = input.split_once("\n\n").unwrap();
    let mut template = template.chars().collect_vec();
    let mut inserted = vec![false; template.len()];
    assert_eq!(template.len(), inserted.len());
    let rules: HashMap<String, char> = unparsed_rules
        .lines()
        .map(|r| r.split_once(" -> ").unwrap())
        .map(|(k, v)| (k.to_string(), v.chars().collect_vec().pop().unwrap()))
        .collect();

    for _ in 1..=10 {
        let mut replacements = 0;
        for i in 0..template.len() - 1 {
            let polymer = template[i].to_string() + &template[i + 1].to_string();
            if let Some(_middle) = rules.get(&polymer) {
                replacements += 1;
            }
        }

        for i in 0..template.len() + replacements - 1 {
            let polymer = template[i].to_string() + &template[i + 1].to_string();
            if !inserted[i] && !inserted[i + 1] {
                if let Some(middle) = rules.get(&polymer) {
                    template.insert(i + 1, *middle);
                    inserted.insert(i + 1, true);
                }
            }
        }

        for b in inserted.iter_mut() {
            *b = false;
        }
    }
    let mut count = HashMap::new();
    for c in template {
        let entry = count.entry(c).or_insert_with(|| 0);
        *entry += 1;
    }
    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();

    max - min
}

pub(crate) fn solve_day14_part2() -> i64 {
    let input = include_str!("../puzzles/day14.txt");
    let (template, unparsed_rules) = input.split_once("\n\n").unwrap();
    let template = template.chars().collect_vec();
    let inserted = vec![false; template.len()];
    assert_eq!(template.len(), inserted.len());
    let rules: HashMap<String, char> = unparsed_rules
        .lines()
        .map(|r| r.split_once(" -> ").unwrap())
        .map(|(k, v)| (k.to_string(), v.chars().collect_vec().pop().unwrap()))
        .collect();

    let mut polymer_map = HashMap::new();

    let mut count = HashMap::new();

    for t in &template {
        let entry = count.entry(*t).or_insert_with(|| 0);
        *entry += 1;
    }

    for i in 0..template.len() - 1 {
        let polymer = template[i].to_string() + &template[i + 1].to_string();
        let entry = polymer_map.entry(polymer).or_insert_with(|| 0_i64);
        *entry += 1;
    }

    for _ in 1_i64..=40 {
        let mut temp_polymer_map = polymer_map.clone();
        for (polymer, occ) in polymer_map {
            if let Some(middle) = rules.get(&polymer) {
                let e = temp_polymer_map.get_mut(&polymer).unwrap();
                *e -= occ;
                let v = polymer.chars().collect_vec();
                let p1 = v[0].to_string() + &middle.to_string();
                let p2 = middle.to_string() + &v[1].to_string();
                let entry1 = temp_polymer_map.entry(p1).or_insert_with(|| 0);
                *entry1 += occ;
                let entry2 = temp_polymer_map.entry(p2).or_insert_with(|| 0);
                *entry2 += occ;

                let entry = count.entry(*middle).or_insert_with(|| 0);
                *entry += occ;
            }
        }
        polymer_map = temp_polymer_map;
    }

    let max = count.values().max().unwrap();
    let min = count.values().min().unwrap();

    max - min
}
