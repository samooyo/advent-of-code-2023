use anyhow::{Context, Result};
use std::fs;
use winnow::{
    ascii::digit1,
    combinator::{alt, delimited, repeat, repeat_till, separated_pair, terminated},
    token::any,
    PResult, Parser as _,
};

#[derive(Debug)]
enum ParseResult {
    Mul { a: u32, b: u32 },
    Do,
    Dont,
}

fn parse_mul(input: &mut &str) -> PResult<ParseResult> {
    let (a, b) = delimited(
        "mul(",
        separated_pair(digit1.parse_to(), ',', digit1.parse_to()),
        ")",
    )
    .parse_next(input)?;
    Ok(ParseResult::Mul { a, b })
}

fn parse_do(input: &mut &str) -> PResult<ParseResult> {
    "do()".parse_next(input).map(|_| ParseResult::Do)
}

fn parse_dont(input: &mut &str) -> PResult<ParseResult> {
    "don't()".parse_next(input).map(|_| ParseResult::Dont)
}

fn parse_instruction(input: &mut &str) -> PResult<ParseResult> {
    let ((), instr) =
        repeat_till(0.., any, alt((parse_mul, parse_do, parse_dont))).parse_next(input)?;
    Ok(instr)
}

fn parse_input(input: &mut &str) -> PResult<Vec<ParseResult>> {
    let instruction = terminated(
        repeat(0.., parse_instruction),
        repeat::<_, _, (), _, _>(0.., any),
    )
    .parse_next(input)?;
    Ok(instruction)
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day03.txt").context("Reading file")?;

    let all_instructions = parse_input.parse(&input).unwrap();

    let mut tot = 0;
    for instr in all_instructions.iter() {
        if let ParseResult::Mul { a, b } = instr {
            tot += a * b
        }
    }
    println!("Part 1: {}", tot);

    tot = 0;
    let mut dododo = true;
    for instr in all_instructions.iter() {
        match instr {
            ParseResult::Mul { a, b } => {
                if dododo {
                    tot += a * b
                }
            }
            ParseResult::Do => dododo = true,
            ParseResult::Dont => dododo = false,
        }
    }
    println!("Part 2: {}", tot);

    Ok(())
}
