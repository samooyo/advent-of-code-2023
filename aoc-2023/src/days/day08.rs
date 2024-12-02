use num::Integer;
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../../inputs/day08.txt");

    let (instructions, map) = input.split_once("\n\n").unwrap();
    let mut hash_map: HashMap<String, (String, String)> = HashMap::new();
    let mut cursor = "AAA".to_string();
    let mut cursors_p2: Vec<String> = vec![];

    for line in map.lines() {
        let (key, value) = line.split_once(" = ").unwrap();
        if key.to_string().ends_with('A') {
            cursors_p2.push(key.to_string())
        }
        let (left, right) = value.split_once(", ").unwrap();
        hash_map.insert(
            key.to_string(),
            (left[1..].to_string(), right[..right.len() - 1].to_string()),
        );
    }

    let mut step = 0;
    for instruction in instructions.chars().cycle() {
        if instruction == 'R' {
            cursor = hash_map.get(&cursor).unwrap().1.clone();
        } else {
            cursor = hash_map.get(&cursor).unwrap().0.clone();
        }
        step += 1;
        if cursor == "ZZZ" {
            break;
        };
    }

    let mut counts: Vec<usize> = vec![];
    for mut cursor in cursors_p2 {
        let mut counter = 0;
        for instruction in instructions.chars().cycle() {
            if instruction == 'R' {
                cursor = hash_map[&cursor].1.clone();
            } else {
                cursor = hash_map[&cursor].0.clone();
            }
            counter += 1;
            if cursor.ends_with('Z') {
                counts.push(counter);
                break;
            }
        }
    }

    println!(
        "Step: p1 = {}, part2 = {}",
        step,
        counts.into_iter().reduce(|x, y| x.lcm(&y)).unwrap()
    );
}
