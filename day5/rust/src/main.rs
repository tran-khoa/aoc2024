use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn turn(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
    fn walk(&self, pos: (isize, isize)) -> (isize, isize) {
        match self {
            Dir::Up => (pos.0 - 1, pos.1),
            Dir::Down => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0, pos.1 - 1),
            Dir::Right => (pos.0, pos.1 + 1),
        }
    }
}

fn full_walk(obstacles: Vec<(isize, isize)>, start: (isize, isize), initial_state: Dir, height: isize, width: isize) -> Option<isize> {
    let mut state = initial_state;
    let mut visited: HashMap<(isize, isize), Vec<Dir>> = HashMap::new();
    let mut pos = start;
    visited.insert(
        pos, vec![state]
    );

    loop {
        let next_pos = state.walk(pos);
        if next_pos.0 < 0 || next_pos.0 >= height || next_pos.1 < 0 || next_pos.1 >= width {
            break;
        }
        if obstacles.contains(&next_pos) {
            state = state.turn();
        } else {
            if visited.contains_key(&next_pos) {
                if visited.get(&next_pos).unwrap().contains(&state) {
                    return None;
                }
                visited.get_mut(&next_pos).unwrap().push(state);
            } else {
                visited.insert(next_pos, vec![state]);
            }
            pos = next_pos;
        }
    }
    Some(visited.len() as isize)
}

fn part1(map_str: &String) -> isize {
    let mut obstacles: Vec<(isize, isize)> = Vec::new();
    let mut start: Result<(isize, isize), &str> = Err("No start position found");
    let height = map_str.lines().count() as isize;
    assert!(height > 0);
    let width = map_str.lines().next().unwrap().chars().count() as isize;

    for (row, line) in map_str.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.push((row as isize, col as isize));
            } else if c == '^' {
                start = Ok((row as isize, col as isize));
            }
        }
    }

    let step_count = full_walk(obstacles, start.unwrap(), Dir::Up, height, width).unwrap();
    step_count
    // 4967
}

fn part2_inefficient(map_str: &String) {
    let mut obstacles: Vec<(isize, isize)> = Vec::new();
    let mut start: Result<(isize, isize), &str> = Err("No start position found");
    let height = map_str.lines().count() as isize;
    assert!(height > 0);
    let width = map_str.lines().next().unwrap().chars().count() as isize;

    for (row, line) in map_str.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                obstacles.push((row as isize, col as isize));
            } else if c == '^' {
                start = Ok((row as isize, col as isize));
            }
        }
    }
    let mut state = Dir::Up;
    let mut pos = start.unwrap();
    let mut possible_obstacles: HashSet<(isize, isize)> = HashSet::new();

    loop {
        let next_pos = state.walk(pos);
        if next_pos.0 < 0 || next_pos.0 >= height || next_pos.1 < 0 || next_pos.1 >= width {
            break;
        }

        if obstacles.contains(&next_pos) {
            state = state.turn();
        } else {
            // assume next_pos is obstacle
            if !possible_obstacles.contains(&next_pos) {
                let mut sub_obstacles = obstacles.clone();
                sub_obstacles.push(next_pos);
                if full_walk(sub_obstacles, pos, state, height, width).is_none() {
                    possible_obstacles.insert(next_pos);
                }
            }
            pos = next_pos;
        }
    }
    println!("{:?}", possible_obstacles);
    println!("Part 2 result: {}", possible_obstacles.len());
    assert!(possible_obstacles.len() < 1831);
    //
}

fn main() {
    let map_str: String = match fs::read_to_string("map.txt") {
        Ok(map) => map,
        Err(e) => panic!("Failed to read map.txt: {}", e),
    };
    println!("Part 1 result: {}", part1(&map_str));
    part2_inefficient(&map_str);
}
