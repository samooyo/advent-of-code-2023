use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::fs;

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day01.txt").context("Reading file")?;
    let mut left_locations: Vec<usize> = Vec::new();
    let mut right_locations: Vec<usize> = Vec::new();

    for line in input.lines() {
        let (left, right) = line
            .split_whitespace()
            .collect_tuple()
            .ok_or_else(|| anyhow!("Couldn't find separator"))
            .context("Parsing line")?;

        left_locations.push(left.parse()?);
        right_locations.push(right.parse()?);
    }

    part1(left_locations.clone(), right_locations.clone());
    part2(left_locations, right_locations);
    Ok(())
}

fn part1(mut left_locations: Vec<usize>, mut right_locations: Vec<usize>) {
    left_locations.sort();
    right_locations.sort();

    let tot: usize = left_locations
        .iter()
        .zip(right_locations.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    println!("Part 1: {}", tot);
}

fn part2(left_locations: Vec<usize>, right_locations: Vec<usize>) {
    let mut tot = 0;

    left_locations.iter().for_each(|l_loc| {
        let nb = right_locations
            .iter()
            .filter(|&r_loc| r_loc == l_loc)
            .count();
        tot += nb * l_loc;
    });

    println!("Part 2: {}", tot);
}
