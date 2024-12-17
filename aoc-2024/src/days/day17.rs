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

    let (registers, instructions) = input.split_once("\n\nProgram: ").unwrap();

    registers.lines().for_each(|line| {
        let (register, value) = line.split_once(": ").unwrap();
        match register {
            "Register A" => registry.a = value.parse().unwrap(),
            "Register B" => registry.b = value.parse().unwrap(),
            "Register C" => registry.c = value.parse().unwrap(),
            _ => panic!("Unknown register"),
        }
    });

    let instructions: Vec<i32> = instructions
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    while registry.pointer < instructions.len() {
        let opcode = Opcode::from_nb(instructions[registry.pointer] as usize);
        let operand = instructions[registry.pointer + 1] as usize;
        opcode.execute(&mut registry, operand);
    }

    registry.result.pop();
    println!("{}", registry.result);
    Ok(())
}
