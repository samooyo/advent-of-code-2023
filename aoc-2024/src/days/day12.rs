use anyhow::{Context, Result};
use std::{collections::HashSet, fs};

#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
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

fn search_directions(
    coordinates: &[Vec<char>],
    coord: (usize, usize),
    plot_done: HashSet<(usize, usize)>,
) -> (Vec<Direction>, usize) {
    let (x, y) = coord;
    let plot = coordinates[y][x];
    let mut next_directions = Vec::new();
    let mut fences = 0;

    for direction in [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ] {
        let (dx, dy) = direction.next();
        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        if next_x >= 0
            && next_y >= 0
            && next_x < coordinates[0].len() as i32
            && next_y < coordinates.len() as i32
            && coordinates[next_y as usize][next_x as usize] == plot
            && !plot_done.contains(&(next_x as usize, next_y as usize))
        {
            next_directions.push(direction);
        }
        if next_x < 0
            || next_x >= coordinates[0].len() as i32
            || next_y < 0
            || next_y >= coordinates.len() as i32
            || coordinates[next_y as usize][next_x as usize] != plot
        {
            fences += 1;
        }
    }

    (next_directions, fences)
}

fn parse_neighbors(
    coordinates: Vec<Vec<char>>,
    coord: (usize, usize),
    plot_done: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let (x, y) = coord;

    let mut possible_paths = Vec::new();
    possible_paths.push((x as i32, y as i32));

    let mut fences = 0;
    let mut area = 0;

    while let Some((x2, y2)) = possible_paths.pop() {
        plot_done.insert((x2 as usize, y2 as usize));

        let (next_directions, temp_fences) =
            search_directions(&coordinates, (x2 as usize, y2 as usize), plot_done.clone());
        fences += temp_fences;

        for direction in &next_directions {
            let (dx, dy) = direction.next();
            let next_x = x2 + dx;
            let next_y = y2 + dy;

            if !possible_paths.contains(&(next_x, next_y)) {
                possible_paths.push((next_x, next_y));
            }
        }

        area += 1;
    }

    (area, fences)
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day12.txt").context("Reading file")?;

    let coordinates: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut plot_done: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<(usize, usize)> = Vec::new();

    for (y, coord_line) in coordinates.iter().enumerate() {
        for (x, _) in coord_line.iter().enumerate() {
            if !plot_done.contains(&(x, y)) {
                regions.push(parse_neighbors(coordinates.clone(), (x, y), &mut plot_done));
            }
        }
    }

    let mut part1 = 0;
    for region in regions {
        part1 += region.0 * region.1;
    }

    println!("{:?}", part1);
    Ok(())
}
