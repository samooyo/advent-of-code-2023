use anyhow::{Context, Result};
use std::fs;

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/test.txt").context("Reading file")?;
    println!("{}", input);

    let part1 = 0;
    let part2 = 0;

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}
