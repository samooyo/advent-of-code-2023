use anyhow::{Context, Result};
use std::fs;

#[derive(Debug)]
struct Play {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
    prize_part2: (isize, isize),
}

impl Play {
    fn from_string(s: &str) -> Self {
        let mut lines = s.lines();
        let parse_coordinates = |line: &str| {
            let mut coords = (0, 0);
            for word in line.split_whitespace() {
                if word.starts_with('X') {
                    coords.0 = word[2..word.len() - 1].parse().unwrap();
                } else if word.starts_with('Y') {
                    coords.1 = word[2..].parse().unwrap();
                }
            }
            coords
        };

        let button_a = parse_coordinates(lines.next().unwrap());
        let button_b = parse_coordinates(lines.next().unwrap());
        let prize = parse_coordinates(lines.next().unwrap());
        let prize_part2 = (prize.0 + 10_000_000_000_000, prize.1 + 10_000_000_000_000);

        Play {
            button_a,
            button_b,
            prize,
            prize_part2,
        }
    }

    fn calculate_tokens(&self, prize: (isize, isize)) -> isize {
        let det = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        if det == 0 {
            return 0;
        }

        let a = (prize.0 * self.button_b.1 - prize.1 * self.button_b.0) / det;
        let b = (prize.0 - a * self.button_a.0) / self.button_b.0;

        if a * self.button_a.0 + b * self.button_b.0 == prize.0
            && a * self.button_a.1 + b * self.button_b.1 == prize.1
        {
            a * 3 + b
        } else {
            0
        }
    }
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day13.txt").context("Reading file")?;

    let plays: Vec<Play> = input.split("\n\n").map(Play::from_string).collect();

    let mut tokens = 0;
    let mut tokens_part2 = 0;

    for play in plays {
        tokens += play.calculate_tokens(play.prize);
        tokens_part2 += play.calculate_tokens(play.prize_part2);
    }

    println!("Part 1: {}", tokens);
    println!("Part 2: {}", tokens_part2);
    Ok(())
}
