use anyhow::{anyhow, Context, Result};
use std::fs;

fn check(cur: usize, numbers: &[usize], i: usize) -> bool {
    if i == 0 {
        return cur == numbers[0];
    }
    if cur % numbers[i] == 0 && check(cur / numbers[i], numbers, i - 1) {
        // number is divisible
        return true;
    }
    if cur >= numbers[i] && check(cur - numbers[i], numbers, i - 1) {
        // we can subtract
        return true;
    }

    false
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day07.txt").context("Reading file")?;

    let operations = input
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(": ").ok_or(anyhow!("Error"))?;
            let p1 = p1.parse::<usize>()?;
            let p2: Vec<usize> = p2
                .split_whitespace()
                .map(|item| item.parse::<usize>().unwrap())
                .collect();
            Ok((p1, p2))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut total = 0;
    for operation in operations.iter() {
        if check(operation.0, &operation.1, operation.1.len() - 1) {
            total += operation.0;
        }
    }

    println!("{:?}", total);
    Ok(())
}
