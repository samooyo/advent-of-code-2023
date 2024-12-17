use anyhow::{Context, Result};
use std::fs;

#[derive(Debug)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, Clone)]
struct Registry {
    a: usize,
    b: usize,
    c: usize,
    pointer: usize,
    result: String,
}

fn combo(combo: usize, registry: &Registry) -> usize {
    if combo <= 3 {
        combo
    } else if combo == 4 {
        registry.a
    } else if combo == 5 {
        registry.b
    } else if combo == 6 {
        registry.c
    } else {
        panic!("Unknown combo")
    }
}

impl Opcode {
    fn from_nb(nb: usize) -> Self {
        match nb {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Unknown opcode"),
        }
    }

    fn execute(&self, registry: &mut Registry, operand: usize) {
        match self {
            // 0 => A = A / 2^COMBO
            Self::Adv => {
                registry.a /= 2usize.pow(combo(operand, registry) as u32);
            }
            // 1 => B = B XOR OPERAND
            Self::Bxl => {
                registry.b ^= operand;
            }
            // 2 => B = COMBO % 8
            Self::Bst => {
                registry.b = combo(operand, registry) % 8;
            }
            // 3 => if A=0 do nothing, else jump to OPERAND
            Self::Jnz => {
                if registry.a != 0 {
                    registry.pointer = operand;
                    return;
                }
            }
            // 4 => B = B XOR C
            Self::Bxc => {
                registry.b ^= registry.c;
            }
            // 5 => output COMBO % 8
            Self::Out => {
                let temp_registry = registry.clone();
                registry
                    .result
                    .push_str(&format!("{},", combo(operand, &temp_registry) % 8));
            }
            // 6 => B = A / 2^COMBO
            Self::Bdv => {
                registry.b = registry.a / 2usize.pow(combo(operand, registry) as u32);
            }
            // 7 => C = A / 2^COMBO
            Self::Cdv => {
                registry.c = registry.a / 2usize.pow(combo(operand, registry) as u32);
            }
        }
        registry.pointer += 2;
    }
}

pub fn run() -> Result<()> {
    let input = fs::read_to_string("inputs/day17.txt").context("Reading file")?;

    let mut registry = Registry {
        a: 0,
        b: 0,
        c: 0,
        pointer: 0,
        result: String::new(),
    };

    let (registers, str_instructions) = input.split_once("\n\nProgram: ").unwrap();

    registers.lines().for_each(|line| {
        let (register, value) = line.split_once(": ").unwrap();
        match register {
            "Register A" => registry.a = value.parse().unwrap(),
            "Register B" => registry.b = value.parse().unwrap(),
            "Register C" => registry.c = value.parse().unwrap(),
            _ => panic!("Unknown register"),
        }
    });

    let instructions: Vec<i32> = str_instructions
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let part_1 = solve(&mut registry.clone(), &instructions);
    let part_2 = solve_part2(&registry, &instructions, str_instructions.to_string());

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
    Ok(())
}

fn common_end(a: String, b: &str) -> usize {
    a.chars()
        .rev()
        .zip(b.chars().rev())
        .take_while(|(a, b)| a == b)
        .count()
}

fn solve(registry: &mut Registry, instructions: &[i32]) -> String {
    while registry.pointer < instructions.len() {
        let opcode = Opcode::from_nb(instructions[registry.pointer] as usize);
        let operand = instructions[registry.pointer + 1] as usize;
        opcode.execute(registry, operand);
    }
    let mut res = registry.result.clone();
    res.pop();
    res
}

fn solve_part2(registry: &Registry, instructions: &[i32], str_instructions: String) -> usize {
    let mut a = 500000000000000;
    let mut tmp_registry = Registry {
        a,
        b: 0,
        c: 0,
        pointer: 0,
        result: String::new(),
    };

    let mut default_common_end = common_end(
        solve(&mut tmp_registry.clone(), instructions),
        &str_instructions,
    );

    while tmp_registry.result != str_instructions {
        let mut default_delta = 1;
        let mut delta = default_delta;

        loop {
            if delta > a {
                default_delta += 1;
                delta = default_delta;
            }

            let mut test_higher = registry.clone();
            test_higher.a = a + delta;
            let mut test_lower = registry.clone();
            if delta > a {
                test_lower.a = 0;
            } else {
                test_lower.a = a - delta;
            }

            let res_higher = solve(&mut test_higher, instructions);
            let res_lower = solve(&mut test_lower, instructions);
            let common_ends_higher = common_end(res_higher.clone(), &str_instructions);
            let common_ends_lower = common_end(res_lower.clone(), &str_instructions);

            if common_ends_higher > default_common_end {
                a += delta;
                default_common_end = common_ends_higher;
                tmp_registry.result = res_higher;
                break;
            } else if common_ends_lower > default_common_end {
                a -= delta;
                default_common_end = common_ends_lower;
                tmp_registry.result = res_lower;
                break;
            } else {
                delta *= 2;
            }
        }
    }

    a
}
