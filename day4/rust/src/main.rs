use std::error::Error;
use candle_core::DType::{F32, U32};
use candle_core::Device::Cpu;
use candle_core::{Module, Tensor};
use candle_nn::conv::{Conv2d, Conv2dConfig};
use itertools::iproduct;
use std::cmp::max;
use std::collections::HashMap;

fn part1(puzzle: &Vec<Vec<char>>) -> u32 {
    let [rows, cols] = [puzzle.len(), puzzle[0].len()];
    // We have negative movement, but leaving the puzzle to the left would mean we drop to the previous row instead,
    // if we do not pad the base.
    let base = max(rows, cols) + 4;
    let puzzle: HashMap<usize, char> = iproduct!(0..rows, 0..cols)
        .map(|(row, col)| (row * base + col, puzzle[row][col]))
        .collect();
    let movements = [
        base,     // down
        1,        // right
        base + 1, // down-right
        base - 1, // down-left
    ];
    iproduct!(0..rows, 0..cols)
        .map(|(row, col)| {
            let current_idx = row * base + col;
            let target = match puzzle[&current_idx] {
                'X' => "XMAS",
                'S' => "SAMX",
                _ => return 0,
            };
            return movements
                .map(|m| {
                    for i in (1..4).rev() {
                        if let Some(letter) = puzzle.get(&(current_idx + m * i)) {
                            if letter == &target.chars().nth(i).unwrap() {
                                continue;
                            }
                        }
                        return 0;
                    }
                    return 1;
                })
                .iter()
                .sum();
        })
        .sum()
}

fn part2(puzzle: &Vec<Vec<char>>) -> Result<u32, Box<dyn Error>> {
    let base: u32 = 4;
    let [rows, cols] = [puzzle.len(), puzzle[0].len()];
    let puzzle: Vec<Vec<u32>> = puzzle
        .iter()
        .map(|row| {
            row.iter()
                .map(|&c| match c {
                    'M' => 1,
                    'A' => 2,
                    'S' => 3,
                    _ => 0,
                })
                .collect()
        })
        .collect();
    let puzzle_tensor = Tensor::new(puzzle, &Cpu)?
        .expand((1, 1, rows, cols))?
        .to_dtype(F32)?;
    let kernel: Vec<Vec<u32>> = vec![
        vec![base.pow(0), 0, base.pow(1)],
        vec![0, base.pow(2), 0],
        vec![base.pow(3), 0, base.pow(4)],
    ];
    let kernel = Tensor::new(kernel, &Cpu)?
        .expand((1, 1, 3, 3))?
        .to_dtype(F32)?;
    let conv2d = Conv2d::new(kernel, None, Conv2dConfig::default());
    let conv_out = conv2d.forward(&puzzle_tensor)?;
    let result = [997u32, 487, 877, 367]
        .map(|target| {
            conv_out
                .to_dtype(U32)
                .unwrap()
                .broadcast_eq(
                    &Tensor::new(vec![target], &Cpu)
                        .unwrap()
                        .expand((1, 1, 1, 1))
                        .unwrap(),
                )
                .unwrap()
                .to_dtype(U32)
                .unwrap()
                .sum_all()
                .unwrap()
                .to_scalar()
                .unwrap()
        })
        .iter()
        .sum();
    Ok(result)
}
fn main() {
    let puzzle: Vec<Vec<char>> = match std::fs::read_to_string("../puzzle.txt") {
        Ok(inputs) => inputs.lines().map(|line| line.chars().collect()).collect(),
        Err(_) => panic!("Error reading file"),
    };
    println!("Part 1: {}", part1(&puzzle));
    println!("Part 2: {}", part2(&puzzle).unwrap());
}
