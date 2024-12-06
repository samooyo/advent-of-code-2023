use anyhow::{Context, Result};
use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn move_in_dir(
    coordinates: &mut [Vec<char>],
    i: usize,
    j: usize,
    dir: &Direction,
) -> ((usize, usize), bool) {
    let (dx, dy) = match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    let mut x = i as isize;
    let mut y = j as isize;
    let rows = coordinates.len() as isize;
    let cols = coordinates[0].len() as isize;

    loop {
        x += dx;
        y += dy;

        if x < 0 || x >= rows || y < 0 || y >= cols {
            return ((0, 0), true);
        }

        if coordinates[x as usize][y as usize] == '#' {
            return (((x - dx) as usize, (y - dy) as usize), false);
        }

        coordinates[x as usize][y as usize] = '0';
    }
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/test.txt").context("Reading file")?;

    let mut coordinates: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    for (i, row) in coordinates.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'^' {
                start = (i, j);
                break;
            }
        }
    }

    coordinates[start.0][start.1] = '0';
    let coordinates2 = coordinates.clone();
    let mut pos = start;
    let mut dir = Direction::Up;

    while let (new_pos, false) = move_in_dir(&mut coordinates, pos.0, pos.1, &dir) {
        pos = new_pos;
        dir = dir.next();
    }

    let mut part2 = 0;
    for (i, row) in coordinates2.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '.' {
                let mut test_coordinates = coordinates2.clone();
                test_coordinates[i][j] = '#';
                pos = start;
                dir = Direction::Up;
                let mut last_obstacles: Vec<(usize, usize)> = Vec::new();

                while let (new_pos, false) = move_in_dir(&mut test_coordinates, pos.0, pos.1, &dir)
                {
                    if last_obstacles.contains(&new_pos) && new_pos != pos {
                        part2 += 1;
                        break;
                    }
                    last_obstacles.push(new_pos);
                    pos = new_pos;
                    dir = dir.next();
                }
            }
        }
    }

    let part1 = coordinates.iter().flatten().filter(|&&c| c == '0').count();

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
    Ok(())
}
