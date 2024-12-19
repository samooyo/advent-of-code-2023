use anyhow::{Context, Result};
use std::{collections::HashMap, fs};

fn available_in_design(design: &str, available_towels: Vec<String>) -> Vec<String> {
    let mut available_towels_in_design: Vec<String> = Vec::new();

    for towel in available_towels {
        if design.contains(&towel) {
            available_towels_in_design.push(towel);
        }
    }

    available_towels_in_design
}

fn nb_of_possible_design(
    design: &str,
    available_towels_in_design: &Vec<String>,
    known_designs: &mut HashMap<String, usize>,
) -> usize {
    if known_designs.contains_key(design) {
        return *known_designs.get(design).unwrap();
    }

    let mut count = 0;
    for towel in available_towels_in_design {
        if towel == design {
            count += 1;
        }

        let next_design = design.strip_prefix(towel);
        if next_design.is_some() {
            count += nb_of_possible_design(
                next_design.unwrap(),
                available_towels_in_design,
                known_designs,
            );
        }
    }

    known_designs.insert(design.to_string(), count);
    count
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day19.txt").context("Reading file")?;

    let (available_towels, designs) = input.split_once("\n\n").unwrap();

    let available_towels: Vec<String> = available_towels
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs: Vec<String> = designs.lines().map(|s| s.to_string()).collect();

    let mut known_designs: HashMap<String, usize> = HashMap::new();
    let mut part1 = 0;
    let mut part2 = 0;
    for design in designs {
        // let available_towels_in_design = available_in_design(&design, available_towels.clone());
        let nb = nb_of_possible_design(&design, &available_towels, &mut known_designs);
        part1 += (nb > 0) as usize;
        part2 += nb;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
