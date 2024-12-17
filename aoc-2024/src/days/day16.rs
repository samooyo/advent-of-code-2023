use anyhow::{Context, Result};
use std::{collections::HashSet, fs};
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

fn search_directions(coordinates: &[Vec<i32>], x: usize, y: usize) -> Vec<&Direction> {
    let next = coordinates[y][x] + 1;

    [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ]
    .iter()
    .filter(move |direction| {
        let (dx, dy) = direction.next();
        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        next_x >= 0
            && next_y >= 0
            && next_x < coordinates[0].len() as i32
            && next_y < coordinates.len() as i32
            && coordinates[next_y as usize][next_x as usize] == next
    })
    .collect::<Vec<&Direction>>()
}

fn search_end(coordinates: &[Vec<i32>], x: usize, y: usize) -> (usize, usize) {
    let mut possible_paths = Vec::new();
    possible_paths.push((x as i32, y as i32));
    let mut ends_part1: HashSet<(i32, i32)> = HashSet::new();
    let mut ends_part2 = Vec::new();

    while let Some((x2, y2)) = possible_paths.pop() {
        let next_directions = search_directions(coordinates, x2 as usize, y2 as usize);

        for direction in &next_directions {
            let (dx, dy) = direction.next();
            let next_x = x2 + dx;
            let next_y = y2 + dy;

            if next_x < 0
                || next_x >= coordinates[0].len() as i32
                || next_y < 0
                || next_y >= coordinates.len() as i32
            {
                break;
            }

            if coordinates[next_y as usize][next_x as usize] == 9 {
                ends_part1.insert((next_x, next_y));
                ends_part2.push((next_x, next_y));
            }

            if !possible_paths.contains(&(next_x, next_y)) {
                possible_paths.push((next_x, next_y));
            }
        }
    }
    (ends_part1.len(), ends_part2.len())
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day10.txt").context("Reading file")?;

    let coordinates = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut scores_part1 = Vec::new();
    let mut scores_part2 = Vec::new();

    for (y, coord) in coordinates.iter().enumerate() {
        for (x, c) in coord.iter().enumerate() {
            if *c == 0 {
                let (part1, part2) = search_end(&coordinates, x, y);
                scores_part1.push(part1);
                scores_part2.push(part2);
            }
        }
    }

    println!("{:?}", scores_part1.iter().sum::<usize>());
    println!("{:?}", scores_part2.iter().sum::<usize>());
    Ok(())
}
