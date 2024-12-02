use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs;

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day02.txt").context("Reading file")?;

    let (mut part1, mut part2) = (0, 0);

    for report in input.lines() {
        let levels = report
            .split_whitespace()
            .collect_vec()
            .iter()
            .map(|x| x.parse::<usize>().unwrap()) // remove unwrap
            .collect_vec();

        if is_safe(levels.clone()) {
            part1 += 1
        }
        if is_safe2(levels) {
            part2 += 1
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn is_safe2(levels: Vec<usize>) -> bool {
    (0..levels.len()).any(|i| {
        let mut cloned_levels = levels.clone();
        cloned_levels.remove(i);
        is_safe(cloned_levels)
    })
}

fn is_safe(levels: Vec<usize>) -> bool {
    let is_bigger = levels[1] > levels[0];

    for (i, level) in levels.iter().enumerate() {
        if (i + 1) == levels.len() {
            return true;
        } else if (is_bigger && &levels[i + 1] < level)
            || (!is_bigger && &levels[i + 1] > level)
            || level.abs_diff(levels[i + 1]) > 3
            || level == &levels[i + 1]
        {
            return false;
        }
    }
    true
}
