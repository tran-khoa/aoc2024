use std::collections::HashMap;

type StoneBlinkPair = (u64, u64);
type Cache = HashMap<StoneBlinkPair, u64>;

fn blink(stone: u64, blinks_left: u64, cache: &mut Cache) -> u64 {
    if let Some(&result) = cache.get(&(stone, blinks_left)) {
        return result;
    }

    if blinks_left == 0 {
        return 1;
    }
    if stone == 0 {
        let result = blink(1, blinks_left - 1, cache);
        cache.insert((stone, blinks_left), result);
        return result;
    }
    let stone_string = stone.to_string();
    if stone_string.len() % 2 == 0 {
        let (left, right) = stone_string.split_at(stone_string.len() / 2);
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        let result = blink(left, blinks_left - 1, cache) + blink(right, blinks_left - 1, cache);
        cache.insert((stone, blinks_left), result);
        return result;
    }
    let result = blink(stone * 2024, blinks_left - 1, cache);
    cache.insert((stone, blinks_left), result);
    result
}

fn part1(stones: &Vec<u64>) -> u64 {
    let mut cache = Cache::new();
    stones.iter().map(|x| blink(*x, 25, &mut cache)).sum()
}

fn part2(stones: &Vec<u64>) -> u64 {
    let mut cache = Cache::new();
    stones.iter().map(|x| blink(*x, 75, &mut cache)).sum()
}

fn main() {
    let input = "3028 78 973951 5146801 5 0 23533 857";
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Part 1: {}", part1(&stones));
    println!("Part 2: {}", part2(&stones));
}
