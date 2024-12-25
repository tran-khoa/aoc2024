use std::collections::{HashMap, VecDeque};

fn evolve(number: i64) -> i64 {
    let number = ((number * 64) ^ number) % 16777216;
    let number = ((number / 32) ^ number) % 16777216;
    let number = ((number * 2048) ^ number) % 16777216;
    number
}

fn part1(buyers: &Vec<i64>) -> i64 {
    buyers
        .iter()
        .map(|buyer_n| {
            let mut buyer_n = *buyer_n;
            for _ in 0..2000 {
                buyer_n = evolve(buyer_n);
            }
            return buyer_n;
        })
        .sum()
}

fn part2(buyers: &Vec<i64>) -> i64 {
    *buyers
        .iter()
        .map(|buyer_n| {
            let mut buying_sequences: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
            let mut prev_price: Option<i64> = None;
            let mut diffs: VecDeque<i64> = VecDeque::new();
            let mut buyer_n = *buyer_n;
            for _ in 0..2000 {
                let price = buyer_n % 10;
                if let Some(prev_price) = prev_price {
                    diffs.push_back(price - prev_price);
                    if diffs.len() > 4 {
                        let quadtup = (diffs[0], diffs[1], diffs[2], diffs[3]);
                        if !buying_sequences.contains_key(&quadtup) {
                            buying_sequences.insert(quadtup, prev_price);
                        }
                        diffs.pop_front();
                    }
                }
                prev_price = Some(price);
                buyer_n = evolve(buyer_n);
            }
            buying_sequences
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<(i64, i64, i64, i64), i64>,
             buying_sequences: HashMap<(i64, i64, i64, i64), i64>| {
                for (key, value) in buying_sequences {
                    *acc.entry(key).or_insert(0) += value;
                }
                acc
            },
        )
        .iter()
        .max_by_key(|(_, &value)| value)
        .unwrap()
        .1
}

fn main() {
    let numbers: Vec<i64> = std::fs::read_to_string("../inputs.txt")
        .map(|file| file.lines().map(|line| line.parse().unwrap()).collect())
        .unwrap();
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}
