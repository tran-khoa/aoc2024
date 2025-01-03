use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn part1(map: &str, positions: &HashMap<char, Vec<(usize, usize)>>) -> usize {
    let height = map.lines().count();
    let width = map.lines().next().unwrap().len();

    positions
        .iter()
        .flat_map(|(_, pos)| {
            pos.iter()
                .permutations(2)
                .map(|ps| {
                    let [p1, p2] = [ps[0], ps[1]];
                    let (dy, dx) = (p2.0 as i32 - p1.0 as i32, p2.1 as i32 - p1.1 as i32);
                    let p_new = (p2.0 as i32 + dy, p2.1 as i32 + dx);
                    if p_new.0 >= 0
                        && p_new.0 < height as i32
                        && p_new.1 >= 0
                        && p_new.1 < width as i32
                    {
                        return Some((p_new.0 as usize, p_new.1 as usize));
                    }
                    None
                })
                .filter(|x| x.is_some())
        })
        .unique()
        .count()
}

fn part2(map: &str, positions: &HashMap<char, Vec<(usize, usize)>>) -> usize {
    let height = map.lines().count();
    let width = map.lines().next().unwrap().len();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for char_positions in positions.values() {
        for perm in char_positions.iter().permutations(2) {
            let (p1, p2) = (*perm[0], *perm[1]);
            let (dy, dx) = (p2.0 as i32 - p1.0 as i32, p2.1 as i32 - p1.1 as i32);
            let mut p_prev = (p2.0 as i32, p2.1 as i32);
            antinodes.insert((p2.0, p2.1));
            loop {
                let p_new = (p_prev.0 + dy, p_prev.1 + dx);
                if p_new.0 >= 0 && p_new.0 < height as i32 && p_new.1 >= 0 && p_new.1 < width as i32
                {
                    antinodes.insert((p_new.0 as usize, p_new.1 as usize));
                    p_prev = p_new;
                } else {
                    break;
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let map = std::fs::read_to_string("../map.txt").unwrap();
    let mut positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    map.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            if c == '.' {
                return;
            }
            positions.entry(c).or_default().push((row, col));
        })
    });

    println!("Part 1: {}", part1(&map, &positions));
    println!("Part 2: {}", part2(&map, &positions));
}
