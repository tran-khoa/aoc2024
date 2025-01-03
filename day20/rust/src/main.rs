use std::collections::HashMap;
use std::ops::{Add, Index, Sub};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Position(i32, i32);
impl Position {
    fn from_tuple(t: (i32, i32)) -> Self {
        Self(t.0, t.1)
    }
    fn l1(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}
impl Index<usize> for Position {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Invalid index"),
        }
    }
}
impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug)]
struct Maze {
    maze: Vec<Vec<char>>,
    start: Position,
    height: i32,
    width: i32,
}

impl Maze {
    fn new(input_str: &str) -> Self {
        let mut start = None;
        let maze: Vec<Vec<char>> = input_str
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Some(Position(x as i32, y as i32))
                        }
                        c
                    })
                    .collect()
            })
            .collect();
        let height = maze.len() as i32;
        let width = maze[0].len() as i32;
        Self {
            maze,
            start: start.unwrap(),
            height,
            width,
        }
    }
    fn find_path(&self) -> Vec<Position> {
        let mut curr = self.start;
        let mut prev: Option<Position> = None;
        let mut path: Vec<Position> = vec![self.start];
        'outer: loop {
            for d in &[(0, -1), (1, 0), (0, 1), (-1, 0)].map(Position::from_tuple) {
                let cand = curr + *d;
                if Some(cand) == prev {
                    continue;
                }
                match self.at(cand) {
                    Some('E') => {
                        path.push(cand);
                        break 'outer;
                    }
                    Some('.') => {
                        path.push(cand);
                        prev = Some(curr);
                        curr = cand;
                        break;
                    }
                    _ => continue,
                }
            }
        }
        path
    }
    fn at(&self, pos: Position) -> Option<&char> {
        if pos.0 < 0 || pos.0 >= self.width || pos.1 < 0 || pos.1 >= self.height {
            return None;
        }
        Some(&self.maze[pos.1 as usize][pos.0 as usize])
    }
}

fn part1(maze: &Maze) -> usize {
    let path = maze.find_path();
    let distance_to_end: HashMap<Position, i32> = path
        .iter()
        .enumerate()
        .map(|(i, pos)| (*pos, (path.len() - 1 - i) as i32))
        .collect();
    let mut viable_cheats = 0;
    for node in path {
        for d in &[(0, -1), (1, 0), (0, 1), (-1, 0)].map(Position::from_tuple) {
            let next = node + *d;
            if let Some('#') = maze.at(next) {
                let next_next = next + *d;
                if let Some('.') = maze.at(next_next) {
                    let distance_gain = distance_to_end[&node] - distance_to_end[&next_next] - 2;
                    viable_cheats += if distance_gain >= 100 { 1 } else { 0 };
                }
            }
        }
    }
    viable_cheats
}

fn part2(maze: &Maze) -> usize {
    let path = maze.find_path();
    let distance_to_end: HashMap<Position, i32> = path
        .iter()
        .enumerate()
        .map(|(i, pos)| (*pos, (path.len() - 1 - i) as i32))
        .collect();
    let mut viable_cheats = 0;
    for (idx, &node) in path.iter().take(path.len() - 100).enumerate() {
        viable_cheats += path
            .iter()
            .skip(idx + 100)
            .map(|n| {
                let dist = (node - *n).l1();
                if dist > 20 {
                    return 0;
                }
                (distance_to_end[&node] - distance_to_end[n] - dist >= 100) as usize
            })
            .sum::<usize>();
    }
    viable_cheats
}

fn main() {
    let maze = Maze::new(&std::fs::read_to_string("../input.txt").unwrap());
    println!("Part 1: {}", part1(&maze));
    println!("Part 2: {}", part2(&maze));
}
