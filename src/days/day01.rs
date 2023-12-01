use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut count = 0;

    if let Ok(lines) = read_lines("./inputs/day01.txt") {
        for line in lines.flatten() {
            let first_digit = line.find(|c: char| c.is_ascii_digit());
            if let Some(m) = &first_digit {
                let a = line.as_bytes()[*m];
                let second_digit = line.rfind(|c: char| c.is_ascii_digit());
                if let Some(n) = &second_digit {
                    let b = line.as_bytes()[*n];
                    let number = (a as char).to_string() + &(b as char).to_string();
                    count += number.parse::<i32>().unwrap();
                }
            }
        }
        println!("And the answer is {}", count)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
