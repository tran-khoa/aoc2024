use ahash::RandomState;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}
impl Op {
    fn apply(&self, x: bool, y: bool) -> bool {
        match self {
            Op::And => x && y,
            Op::Or => x || y,
            Op::Xor => x ^ y,
        }
    }
}
#[derive(Clone)]
struct Instruction {
    op: Op,
    x: String,
    y: String,
    z: String,
}

fn parse_inputs(input: &str) -> (HashMap<String, bool, RandomState>, Vec<Instruction>) {
    let inputs_regex = Regex::new(r"(.{3}): (\d)").unwrap();
    let inputs: HashMap<String, bool, RandomState> = inputs_regex
        .captures_iter(input)
        .par_bridge()
        .map(|cap| (cap[1].to_string(), cap[2].parse::<u8>().unwrap() == 1))
        .collect();
    let instructions_regex = Regex::new(r"(.{3}) (XOR|AND|OR) (.{3}) -> (.{3})").unwrap();
    let instructions: Vec<Instruction> = instructions_regex
        .captures_iter(input)
        .par_bridge()
        .map(|cap| {
            let op = match &cap[2] {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => panic!("Invalid operation"),
            };
            Instruction {
                op,
                x: cap[1].to_string(),
                y: cap[3].to_string(),
                z: cap[4].to_string(),
            }
        })
        .collect();
    (inputs, instructions)
}

fn part1(inputs: &HashMap<String, bool, RandomState>, instructions: &[Instruction]) -> u64 {
    let mut registers = inputs.clone();
    let mut instructions: VecDeque<Instruction> = instructions.iter().cloned().collect();
    while let Some(instruction) = instructions.pop_front() {
        match registers
            .get(&instruction.x)
            .and_then(|&a_val| registers.get(&instruction.y).map(|&b_val| (a_val, b_val)))
        {
            Some((a_val, b_val)) => {
                registers.insert(instruction.z.clone(), instruction.op.apply(a_val, b_val));
            }
            None => instructions.push_back(instruction),
        }
    }
    registers
        .par_iter()
        .filter(|(k, &v)| k.starts_with('z') && v)
        .map(|(k, _)| 2u64.pow(k[1..].parse::<u32>().unwrap()))
        .sum()
}

fn prob_check_carry_or_xor1(instructions: &Vec<Instruction>, name: &str) -> bool {
    let counts = instructions
        .par_iter()
        .filter(|instruct| instruct.x == name || instruct.y == name)
        .map(|instruct| match instruct.op {
            Op::And => (1, 0, 0),
            Op::Xor => (0, 1, 0),
            Op::Or => (0, 0, 1),
        })
        .reduce_with(|acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2))
        .unwrap_or((0, 0, 0));
    counts == (1, 1, 0)
}

fn prob_check_or(instructions: &Vec<Instruction>, name: &str) -> bool {
    instructions
        .par_iter()
        .find_any(|instruct| instruct.op == Op::Or && (instruct.x == name || instruct.y == name))
        .is_some()
}

fn prob_part2(instructions: &Vec<Instruction>) -> String {
    let mut wrongs: Vec<String> = Vec::with_capacity(8);
    // check half-adder
    for instruct in instructions {
        match (
            &instruct.op,
            instruct.x.as_str(),
            instruct.y.as_str(),
            instruct.z.as_str(),
        ) {
            (Op::Xor, "x00", "y00", z) | (Op::Xor, "y00", "x00", z) => {
                if z != "z00" {
                    println!("XOR(x00, y00) -> {}, not z00", z);
                    wrongs.push(z.to_string());
                }
            }
            (Op::And, "x00", "y00", z) | (Op::And, "y00", "x00", z) | (Op::Or, _, _, z) => {
                if z != "z45" && !prob_check_carry_or_xor1(instructions, z) {
                    println!("{} is not a carry or XOR1", z);
                    wrongs.push(z.to_string());
                }
            }
            (Op::Xor, x, y, z) => {
                if x.starts_with('x') && y.starts_with('y')
                    || x.starts_with('y') && y.starts_with('x')
                {
                    if !prob_check_carry_or_xor1(instructions, z) {
                        println!("{} is not a carry or XOR1", z);
                        wrongs.push(z.to_string());
                    }
                } else if !z.starts_with('z') {
                    println!("XOR({}, {}) -> {}, not z??", x, y, z);
                    wrongs.push(z.to_string());
                }
            }
            (Op::And, _, _, z) => {
                if !prob_check_or(instructions, z) {
                    println!("{} is not an OR", z);
                    wrongs.push(z.to_string());
                }
            }
        }
    }
    wrongs.sort();
    wrongs.join(",")
}

fn main() {
    let input = std::fs::read_to_string("../inputs.txt").unwrap();
    let (inputs, instructions) = parse_inputs(&input);
    println!("Part 1: {}", part1(&inputs, &instructions));
    println!("Part 2: {}", prob_part2(&instructions));
}
