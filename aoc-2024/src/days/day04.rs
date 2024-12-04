use anyhow::{Context, Result};
use std::fs;

fn look_down(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if (i + 3) >= coordinates.len() {
        return 0;
    }
    if coordinates[i + 1][j] == 'M' && coordinates[i + 2][j] == 'A' && coordinates[i + 3][j] == 'S'
    {
        return 1;
    }

    0
}

fn look_up(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if i < 3 {
        return 0;
    }
    if coordinates[i - 1][j] == 'M' && coordinates[i - 2][j] == 'A' && coordinates[i - 3][j] == 'S'
    {
        return 1;
    }

    0
}

fn look_right(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if (j + 3) >= coordinates[i].len() {
        return 0;
    }
    if coordinates[i][j + 1] == 'M' && coordinates[i][j + 2] == 'A' && coordinates[i][j + 3] == 'S'
    {
        return 1;
    }

    0
}

fn look_left(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if j < 3 {
        return 0;
    }
    if coordinates[i][j - 1] == 'M' && coordinates[i][j - 2] == 'A' && coordinates[i][j - 3] == 'S'
    {
        return 1;
    }

    0
}

fn look_down_right(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if (i + 3) >= coordinates.len() || (j + 3) >= coordinates[i].len() {
        return 0;
    }
    if coordinates[i + 1][j + 1] == 'M'
        && coordinates[i + 2][j + 2] == 'A'
        && coordinates[i + 3][j + 3] == 'S'
    {
        return 1;
    }

    0
}

fn look_down_left(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if (i + 3) >= coordinates.len() || j < 3 {
        return 0;
    }
    if coordinates[i + 1][j - 1] == 'M'
        && coordinates[i + 2][j - 2] == 'A'
        && coordinates[i + 3][j - 3] == 'S'
    {
        return 1;
    }

    0
}

fn look_up_right(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if i < 3 || (j + 3) >= coordinates[i].len() {
        return 0;
    }
    if coordinates[i - 1][j + 1] == 'M'
        && coordinates[i - 2][j + 2] == 'A'
        && coordinates[i - 3][j + 3] == 'S'
    {
        return 1;
    }

    0
}

fn look_up_left(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    if i < 3 || j < 3 {
        return 0;
    }
    if coordinates[i - 1][j - 1] == 'M'
        && coordinates[i - 2][j - 2] == 'A'
        && coordinates[i - 3][j - 3] == 'S'
    {
        return 1;
    }

    0
}

fn search_word(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    look_down(coordinates, i, j)
        + look_up(coordinates, i, j)
        + look_right(coordinates, i, j)
        + look_left(coordinates, i, j)
        + look_down_right(coordinates, i, j)
        + look_down_left(coordinates, i, j)
        + look_up_right(coordinates, i, j)
        + look_up_left(coordinates, i, j)
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day04.txt").context("Reading file")?;
    let mut coordinates: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        coordinates.push(row);
    }

    let mut xmas_found = 0;
    for (i, row) in coordinates.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'X' {
                xmas_found += search_word(&coordinates, i, j)
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!("XMAS found: {}", xmas_found);

    Ok(())
}
