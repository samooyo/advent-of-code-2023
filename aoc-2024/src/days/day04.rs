use anyhow::{Context, Result};
use std::fs;

const MAS: [char; 3] = ['M', 'A', 'S'];

fn look_in_direction(coordinates: &[Vec<char>], i: usize, j: usize, di: isize, dj: isize) -> usize {
    for (step, expected_char) in MAS.iter().enumerate() {
        let ni = i as isize + (step as isize + 1) * di;
        let nj = j as isize + (step as isize + 1) * dj;
        if ni < 0
            || nj < 0
            || ni as usize >= coordinates.len()
            || nj as usize >= coordinates[0].len()
            || coordinates[ni as usize][nj as usize] != *expected_char
        {
            return 0;
        }
    }
    1
}

fn search_xmas(coordinates: &[Vec<char>], i: usize, j: usize) -> usize {
    let directions = [
        (1, 0),   // Down
        (-1, 0),  // Up
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ];

    directions
        .iter()
        .map(|&(di, dj)| look_in_direction(coordinates, i, j, di, dj))
        .sum()
}

fn search_x_mas(coordinates: &[Vec<char>], i: usize, j: usize) -> bool {
    if i < 1 || j < 1 || i >= coordinates.len() - 1 || j >= coordinates[0].len() - 1 {
        return false;
    }

    let diagonal_1 = (coordinates[i - 1][j - 1], coordinates[i + 1][j + 1]) == ('M', 'S')
        || (coordinates[i - 1][j - 1], coordinates[i + 1][j + 1]) == ('S', 'M');
    let diagonal_2 = (coordinates[i - 1][j + 1], coordinates[i + 1][j - 1]) == ('M', 'S')
        || (coordinates[i - 1][j + 1], coordinates[i + 1][j - 1]) == ('S', 'M');

    diagonal_1 && diagonal_2
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day04.txt").context("Reading file")?;
    let coordinates: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut xmas_found = 0;
    let mut x_mas_found = 0;
    for (i, row) in coordinates.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'X' {
                xmas_found += search_xmas(&coordinates, i, j);
            } else if *c == 'A' && search_x_mas(&coordinates, i, j) {
                x_mas_found += 1;
            }
        }
    }

    println!("Part 1: {}", xmas_found);
    println!("Part 2: {}", x_mas_found);
    Ok(())
}
