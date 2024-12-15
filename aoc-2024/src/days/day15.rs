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

fn print_map(coordinates: &Vec<Vec<char>>) {
    for line in coordinates {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn move_in_map(
    coordinates: &mut Vec<Vec<char>>,
    coord: (usize, usize),
    direction: Direction,
) -> (usize, usize) {
    let (delta_x, delta_y) = direction.next();
    let (next_x, next_y) = (coord.0 as i32 + delta_x, coord.1 as i32 + delta_y);
    if coordinates[next_y as usize][next_x as usize] == '#' {
        return (coord.0, coord.1);
    } else if coordinates[next_y as usize][next_x as usize] == '.' {
        return (next_x as usize, next_y as usize);
    } else if coordinates[next_y as usize][next_x as usize] == 'O' {
        let (mut temp_x, mut temp_y) = (next_x, next_y);

        while coordinates[temp_y as usize][temp_x as usize] == 'O' {
            (temp_x, temp_y) = (temp_x + delta_x, temp_y + delta_y);
        }

        if coordinates[temp_y as usize][temp_x as usize] == '#' {
            return (coord.0, coord.1);
        }
        println!("temp_x: {}, temp_y: {}", temp_x, temp_y);
        println!("next_x: {}, next_y: {}", next_x, next_y);
        coordinates[temp_y as usize][temp_x as usize] = 'O';
        coordinates[next_y as usize][next_x as usize] = '.';
        return (next_x as usize, next_y as usize);
    }
    (coord.0, coord.1)
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day15.txt").context("Reading file")?;

    let parts: Vec<&str> = input.split("\n\n").collect();
    let (map, instructions) = (parts[0], parts[1]);

    let mut coordinates: Vec<Vec<char>> = map
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let (mut pos_x, mut pos_y) = coordinates
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == '@' { Some((x, y)) } else { None })
        })
        .unwrap();

    for instruction in instructions.chars() {
        coordinates[pos_y][pos_x] = '.';
        if instruction == '>' {
            (pos_x, pos_y) = move_in_map(&mut coordinates, (pos_x, pos_y), Direction::Right);
            coordinates[pos_y][pos_x] = '@';
        } else if instruction == '<' {
            (pos_x, pos_y) = move_in_map(&mut coordinates, (pos_x, pos_y), Direction::Left);
            coordinates[pos_y][pos_x] = '@';
        } else if instruction == '^' {
            (pos_x, pos_y) = move_in_map(&mut coordinates, (pos_x, pos_y), Direction::Up);
            coordinates[pos_y][pos_x] = '@';
        } else if instruction == 'v' {
            (pos_x, pos_y) = move_in_map(&mut coordinates, (pos_x, pos_y), Direction::Down);
            coordinates[pos_y][pos_x] = '@';
        }
        println!("{}", instruction);
        print_map(&coordinates);
        println!();
    }

    let mut total = 0;
    for (y, line) in coordinates.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'O' {
                total += y * 100 + x;
            }
        }
    }

    println!("{:?}", total);
    Ok(())
}
