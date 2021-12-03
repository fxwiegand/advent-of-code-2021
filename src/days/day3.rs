use itertools::Itertools;

pub(crate) fn solve_day3() -> u32 {
    let input = include_str!("../puzzles/day3.txt");
    let mut ones = vec![0; 12];
    let mut zeros = vec![0; 12];
    for line in input.lines() {
        for (i, char) in line.chars().enumerate() {
            if char == '1' {
                ones[i] += 1;
            } else {
                zeros[i] += 1;
            }
        }
    }
    let mut gamma = String::new();
    let mut epsilon = String::new();

    for (o, z) in ones.iter().zip_eq(zeros.iter()) {
        if o > z {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

pub(crate) fn solve_day3_part2() -> u32 {
    let input = include_str!("../puzzles/day3.txt");
    let mut potential_oxygen = input.lines().map(|s| s.to_owned()).collect_vec();
    for i in 0..12 {
        let mut ones = 0;
        let mut zeros = 0;
        for s in &potential_oxygen {
            if s.chars().nth(i).unwrap() == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        if ones >= zeros {
            potential_oxygen = potential_oxygen
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '1')
                .collect_vec();
        } else {
            potential_oxygen = potential_oxygen
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '0')
                .collect_vec();
        }
        if potential_oxygen.len() == 1 {
            break;
        }
    }

    let mut potential_co2 = input.lines().map(|s| s.to_owned()).collect_vec();
    for i in 0..12 {
        let mut ones = 0;
        let mut zeros = 0;
        for s in &potential_co2 {
            if s.chars().nth(i).unwrap() == '0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        if ones < zeros {
            potential_co2 = potential_co2
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '1')
                .collect_vec();
        } else {
            potential_co2 = potential_co2
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '0')
                .collect_vec();
        }
        if potential_co2.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(&potential_oxygen.pop().unwrap(), 2).unwrap()
        * u32::from_str_radix(&potential_co2.pop().unwrap(), 2).unwrap()
}
