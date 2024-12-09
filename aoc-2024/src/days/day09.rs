use anyhow::{Context, Result};
use std::fs;

fn parse_inputs(input: &str) -> Vec<Option<usize>> {
    let mut disk: Vec<Option<usize>> = Vec::new();
    let mut file_id = 0;

    for (i, c) in input.chars().enumerate() {
        let nb_repetitions = c.to_digit(10).unwrap();
        for _ in 0..nb_repetitions {
            if i % 2 == 0 {
                disk.push(Some(file_id));
            } else {
                disk.push(None);
            }
        }
        if i % 2 == 0 {
            file_id += 1;
        }
    }
    disk
}

fn check_if_enough_space(disk: &[Option<usize>], nb_of_nb: usize) -> (bool, usize) {
    let mut free_count = 0;
    for (i, &slot) in disk.iter().enumerate() {
        if slot.is_none() {
            free_count += 1;
            if free_count == nb_of_nb {
                return (true, i + 1 - nb_of_nb);
            }
        } else {
            free_count = 0;
        }
    }
    (false, 0)
}

fn part1(disk: &mut Vec<Option<usize>>) {
    let mut i = 0;
    while i < disk.len() {
        if disk[i].is_none() {
            while disk.last().map_or(false, Option::is_none) {
                disk.pop();
            }
            if i >= disk.len() {
                break;
            }
            disk[i] = disk.pop().flatten();
        }
        i += 1;
    }
    println!(
        "Part 1: {}",
        disk.iter()
            .enumerate()
            .map(|(i, nb)| i * nb.unwrap())
            .sum::<usize>()
    );
}

fn part2(disk: &mut [Option<usize>]) {
    let mut last_file_id = disk[disk.len() - 1].unwrap();

    while last_file_id > 0 {
        let nb_of_nb = disk.iter().filter(|&x| x == &Some(last_file_id)).count();
        let (is_enough_space, free_index) = check_if_enough_space(disk, nb_of_nb);

        if is_enough_space
            && disk.iter().position(|&x| x == Some(last_file_id)).unwrap() > free_index
        {
            disk.iter_mut().for_each(|x| {
                if *x == Some(last_file_id) {
                    *x = None;
                }
            });
            disk[free_index..(free_index + nb_of_nb)]
                .iter_mut()
                .for_each(|x| *x = Some(last_file_id));
        }

        last_file_id -= 1;
    }

    println!(
        "Part 2: {}",
        disk.iter()
            .enumerate()
            .map(|(i, nb)| i * nb.unwrap_or(0))
            .sum::<usize>()
    );
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day09.txt").context("Reading file")?;

    let mut disk = parse_inputs(&input);

    part1(&mut disk.clone());
    part2(&mut disk);

    Ok(())
}
