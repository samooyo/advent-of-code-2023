use anyhow::{anyhow, Context, Result};
use std::fs;

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day05.txt").context("Reading file")?;

    let (raw_rules, raw_updates) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Invalid format input"))?;

    let rules_tuple = raw_rules
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once("|").ok_or(anyhow!("Error"))?;
            let p1 = p1.parse::<usize>()?;
            let p2 = p2.parse::<usize>()?;
            Ok((p1, p2))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut updates = raw_updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|num| num.parse::<usize>().map_err(|e| anyhow!(e)))
                .collect()
        })
        .collect::<Result<Vec<Vec<usize>>>>()?;

    let mut part1 = 0;
    let mut part2 = 0;

    for update in updates.iter_mut() {
        if update.is_sorted_by(|a, b| {
            let rule = rules_tuple
                .iter()
                .find(|(p1, p2)| (a == p1 && b == p2) || (a == p2 && b == p1))
                .unwrap();
            &(*a, *b) == rule
        }) {
            part1 += update[update.len() / 2];
        } else {
            update.sort_unstable_by(|a, b| {
                if rules_tuple.contains(&(*a, *b)) {
                    std::cmp::Ordering::Less
                } else if rules_tuple.contains(&(*b, *a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            part2 += update[update.len() / 2];
        }
    }

    println!("{:?}", part1);
    println!("{:?}", part2);

    Ok(())
}
