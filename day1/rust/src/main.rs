use std::collections::HashMap;

fn part1(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    let mut left_list = left_list.clone();
    let mut right_list = right_list.clone();
    left_list.sort();
    right_list.sort();
    
    left_list.iter().zip(right_list.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

fn part2(left_list: &Vec<u32>, right_list: &Vec<u32>) -> u32 {
    let mut counter = HashMap::new();
    for &num in right_list.iter() {
        *counter.entry(num).or_insert(0) += 1;
    }
    left_list.iter()
        .map(|num| counter.get(num).unwrap_or(&0) * num)
        .sum()
}

fn main() {
    let (left_list, right_list): (Vec<u32>, Vec<u32>) = std::fs::read_to_string("../locations.txt").unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("   ").collect();
            (parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap())
        })
        .unzip();
    println!("Part 1: {}", part1(&left_list, &right_list));
    println!("Part 2: {}", part2(&left_list, &right_list));
}
