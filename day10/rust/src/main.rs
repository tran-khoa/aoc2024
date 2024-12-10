use std::collections::{HashMap, HashSet};

fn dfs_unique_ends(
    map: &Vec<Vec<u32>>,
    pos: (usize, usize),
    cache: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>
) {
    if cache.contains_key(&pos) {
        return;
    }
    if map[pos.0][pos.1] == 9 {
        cache.entry(pos).or_insert(HashSet::new()).insert(pos);
        return;
    }
    let next_val = map[pos.0][pos.1] + 1;
    cache.insert(pos, HashSet::new());
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbor_pos = (pos.0 as i32 + dx, pos.1 as i32 + dy);
        if neighbor_pos.0 < 0 || neighbor_pos.0 >= map.len() as i32 || neighbor_pos.1 < 0 || neighbor_pos.1 >= map[0].len() as i32 {
            continue;
        }
        let neighbor_pos = (neighbor_pos.0 as usize, neighbor_pos.1 as usize);
        if map[neighbor_pos.0][neighbor_pos.1] != next_val {
            continue;
        }
        dfs_unique_ends(&map, neighbor_pos, cache);
        let neighbor_res = cache.get(&neighbor_pos).unwrap().clone();
        cache.get_mut(&pos).unwrap().extend(neighbor_res);
    }
}

fn part1(map: &Vec<Vec<u32>>) -> u32 {
    let mut cache: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut score_sum = 0;
    for (i, j) in itertools::iproduct!(0..map.len(), 0..map[0].len()) {
        if map[i][j] == 0 {
            dfs_unique_ends(&map, (i, j), &mut cache);
            score_sum += cache[&(i, j)].len() as u32;
        }
    }
   score_sum
}

fn main() {
    let map: Vec<Vec<u32>> = std::fs::read_to_string("../map.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("Part 1: {}", part1(&map));
}
