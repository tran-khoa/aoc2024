use rayon::prelude::*;

fn parse_inputs(inputs: &str) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let lines: Vec<&str> = inputs.lines().collect();
    let (keys, locks): (Vec<_>, Vec<_>) = lines
        .par_chunks(8)
        .map(|chunk| {
            let is_key = chunk[0].chars().all(|c| c == '.');
            let counts = chunk[1..=5]
                .iter()
                .map(|line| {
                    line.chars()
                        .map(|c| if c == '.' { 0 } else { 1 })
                        .collect::<Vec<_>>()
                })
                .reduce(|acc, x| acc.iter().zip(x.iter()).map(|(a, b)| a + b).collect())
                .unwrap();
            (is_key, counts)
        })
        .partition(|(is_key, _)| *is_key);
    let keys = keys.into_iter().map(|(_, counts)| counts).collect();
    let locks = locks.into_iter().map(|(_, counts)| counts).collect();
    (keys, locks)
}

fn is_lock_key_fit(key: &[i32], lock: &[i32]) -> bool {
    key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5)
}

fn part1(keys: &Vec<Vec<i32>>, locks: &Vec<Vec<i32>>) -> i32 {
    keys.par_iter()
        .map(|key| {
            locks
                .par_iter()
                .filter(|lock| is_lock_key_fit(key, lock))
                .count() as i32
        })
        .sum()
}

fn main() {
    let inputs = std::fs::read_to_string("../inputs.txt").unwrap();
    let (keys, locks) = parse_inputs(&inputs);
    println!("Keys: {:?}", keys);
    println!("Locks: {:?}", locks);
    println!("Part 1: {}", part1(&keys, &locks));
}
