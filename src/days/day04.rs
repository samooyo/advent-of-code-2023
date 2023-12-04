use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut glob_points = 0;

    if let Ok(lines) = read_lines("./inputs/day04.txt") {
        for line in lines.flatten() {
            let mut split = line.split('|');
            let mut full_card = split.next().unwrap().split(':');
            full_card.next();
            let clean_card = full_card.next().unwrap();
            let full_win = split.next().unwrap();

            let card_numbers: Vec<&str> = clean_card.split_whitespace().collect();
            let winning_numbers: Vec<&str> = full_win.split_whitespace().collect();

            let mut points = 0;
            for num in winning_numbers {
                if card_numbers.contains(&num) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            }
            glob_points += points;
        }
    }
    println!("Day 4: {}", glob_points);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
