use anyhow::{Context, Result};
use std::{collections::HashSet, fs};

fn is_inside(
    first_pos: (i32, i32),
    second_pos: (i32, i32),
    boundaries: (i32, i32),
    positions: &mut HashSet<(i32, i32)>,
    part2: bool,
) -> usize {
    let mut ret = 0;
    let y_diff = second_pos.0 - first_pos.0;
    let x_diff = second_pos.1 - first_pos.1;
    let mut y = first_pos.0 - y_diff;
    let mut x = first_pos.1 - x_diff;
    while y >= 0 && x >= 0 && x < boundaries.1 {
        if !positions.contains(&(y, x)) {
            positions.insert((y, x));
            ret += 1;
        }
        if !part2 {
            break;
        }
        y -= y_diff;
        x -= x_diff;
    }
    y = second_pos.0 + y_diff;
    x = second_pos.1 + x_diff;
    while y < boundaries.0 && x >= 0 && x < boundaries.1 {
        if !positions.contains(&(y, x)) {
            positions.insert((y, x));
            ret += 1;
        }
        if !part2 {
            break;
        }
        y += y_diff;
        x += x_diff;
    }
    ret
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day08.txt").context("Reading file")?;

    let coordinates: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = coordinates.len();
    let width = coordinates[0].len();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut positions_part1: HashSet<(i32, i32)> = HashSet::new();
    let mut positions_part2: HashSet<(i32, i32)> = HashSet::new();

    for (i, row) in coordinates.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                for (k, row_k) in coordinates.iter().enumerate().skip(i + 1) {
                    for (l, &c_k) in row_k.iter().enumerate() {
                        if c_k == *c {
                            part1 += is_inside(
                                (i as i32, j as i32),
                                (k as i32, l as i32),
                                (height as i32, width as i32),
                                &mut positions_part1,
                                false,
                            );
                            part2 += is_inside(
                                (i as i32, j as i32),
                                (k as i32, l as i32),
                                (height as i32, width as i32),
                                &mut positions_part2,
                                true,
                            );
                        }
                    }
                }
            }
        }
    }

    for (i, row) in coordinates.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' && !positions_part2.contains(&(i as i32, j as i32)) {
                part2 += 1;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}
