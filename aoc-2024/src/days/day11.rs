use anyhow::{Context, Result};
use std::{collections::HashMap, fs};

fn split_even_digit(num: usize) -> (usize, usize) {
    let binding = num.to_string();
    let (nb1, nb2) = binding.split_at(num.to_string().len() / 2);
    (nb1.parse().unwrap(), nb2.parse().unwrap())
}

fn insert_or_incr(map: &mut HashMap<usize, usize>, item: usize, count: usize) {
    if map.contains_key(&item) {
        let prev_count = map.get(&item).unwrap();
        map.insert(item, count + prev_count);
    } else {
        map.insert(item, count);
    }
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day11.txt").context("Reading file")?;

    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|nb| nb.parse().unwrap())
        .collect();

    let mut map: HashMap<usize, usize> = numbers.iter().map(|&nb| (nb, 1)).collect();
    let mut part1 = 0;

    for j in 0..75 {
        let mut temp_map: HashMap<usize, usize> = HashMap::new();

        for (nb, i) in &map {
            if *nb == 0 {
                insert_or_incr(&mut temp_map, 1, *i);
            } else if nb.to_string().len() % 2 == 0 {
                let (n1, n2) = split_even_digit(*nb);
                insert_or_incr(&mut temp_map, n1, *i);
                insert_or_incr(&mut temp_map, n2, *i);
            } else {
                insert_or_incr(&mut temp_map, nb * 2024, *i);
            }
        }
        map = temp_map;
        if j == 24 {
            for count in map.values() {
                part1 += count;
            }
        }
    }

    let mut part2 = 0;
    for (_, count) in map {
        part2 += count;
    }
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
    Ok(())
}
