use anyhow::{Context, Result};
use std::fs;

fn split_even_digit(num: usize) -> (usize, usize) {
    let binding = num.to_string();
    let (nb1, nb2) = binding.split_at(num.to_string().len() / 2);
    (nb1.parse().unwrap(), nb2.parse().unwrap())
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day11.txt").context("Reading file")?;

    let mut numbers: Vec<usize> = input
        .split_whitespace()
        .map(|nb| nb.parse().unwrap())
        .collect();

    let mut part1 = 0;

    for j in 0..75 {
        let mut i = 0;
        let mut length = numbers.len();

        while i < length {
            if numbers[i] == 0 {
                numbers[i] = 1;
            } else if numbers[i].to_string().len() % 2 == 0 {
                let (n1, n2) = split_even_digit(numbers[i]);
                numbers[i] = n1;
                numbers.insert(i + 1, n2);
                i += 1;
                length += 1;
            } else {
                numbers[i] *= 2024;
            }
            i += 1;
        }
        if j == 24 {
            part1 = numbers.len();
        }
    }

    println!("{:?}", part1);
    println!("{:?}", numbers.len());
    Ok(())
}
