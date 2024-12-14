use ndarray::prelude::*;
use rayon::prelude::*;
use regex::RegexBuilder;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
#[derive(Clone)]
struct Robot {
    position: Array1<i64>,
    velocity: Array1<i64>,
}

fn bfs_component(robots: &Vec<Robot>, root: &Robot, states: &mut HashMap<(i64, i64), bool>, threshold: usize) -> usize {
    let vertices: HashSet<(i64, i64)> = robots
        .iter()
        .map(|v| (v.position[0], v.position[1]))
        .collect();
    let root = (root.position[0], root.position[1]);

    let mut nodes = 0;
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    states.insert(root, true);
    queue.push_back(root);
    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();
        if nodes >= threshold {
            return threshold;
        }
        nodes += 1;
        let neighbors = [
            (node.0 - 1, node.1),
            (node.0 + 1, node.1),
            (node.0, node.1 - 1),
            (node.0, node.1 + 1),
        ];
        for n in neighbors.iter() {
            if vertices.contains(n) && !states[n] {
                states.insert(n.clone(), true);
                queue.push_back(n.clone());
            }
        }
    }
    nodes
}

fn find_component(robots: &Vec<Robot>, threshold: usize) -> bool {
    let vertices: HashSet<(i64, i64)> = robots
        .iter()
        .map(|v| (v.position[0], v.position[1]))
        .collect();
    let mut states: HashMap<(i64, i64), bool> =
        HashMap::from_par_iter(vertices.par_iter().map(|v| (v.clone(), false)));

    for robot in robots.iter() {
        if states[&(robot.position[0], robot.position[1])] {
            continue;
        }
        let nodes = bfs_component(robots, robot, &mut states, threshold);
        if nodes >= threshold {
            return true;
        }
    }
    false
}

fn predict_robots(robots: &Vec<Robot>, height: i64, width: i64, time: i64) -> Vec<Robot> {
    robots
        .par_iter()
        .map(|robot| ((&robot.position + &robot.velocity * time) % array![width, height], robot))
        .map(|(vec, robot)| {
            Robot {
                position: array![
                if vec[0] >= 0 { vec[0] } else { vec[0] + width },
                if vec[1] >= 0 { vec[1] } else { vec[1] + height }
            ],
                velocity: robot.velocity.clone(),
            }
        })
        .collect()
}

fn part1(robots: &Vec<Robot>, height: i64, width: i64) -> i64 {
    predict_robots(robots, height, width, 100)
        .iter()
        .fold(vec![0, 0, 0, 0], |mut acc: Vec<i64>, robot| {
            let x = robot.position[0];
            let y = robot.position[1];
            if x < width / 2 && y < height / 2 {
                acc[0] += 1;
            } else if x > width / 2 && y < height / 2 {
                acc[1] += 1;
            } else if x < width / 2 && y > height / 2 {
                acc[2] += 1;
            } else if x > width / 2 && y > height / 2 {
                acc[3] += 1;
            }
            acc
        })
        .iter()
        .product()
}

fn print_robots(robots: &Vec<Robot>, height: i64, width: i64) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots.iter() {
        grid[robot.position[1] as usize][robot.position[0] as usize] = '#';
    }
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn part2(robots: &Vec<Robot>, height: i64, width: i64) -> i64 {
    let mut time = 0;
    let mut current_robots: Vec<Robot> = (*robots).clone();
    loop {
        time += 1;
        current_robots = predict_robots(&current_robots, height, width, 1);
        if find_component(&current_robots,  25) {
            print_robots(&current_robots, height, width);
            return time;
        }
    }
}

fn main() {
    let robot_regex = RegexBuilder::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)")
        .multi_line(true)
        .build()
        .unwrap();
    let robots: Vec<Robot> = robot_regex
        .captures_iter(include_str!("../robots.txt"))
        .map(|cap| Robot {
            position: array![
                cap[1].parse::<i64>().unwrap(),
                cap[2].parse::<i64>().unwrap()
            ],
            velocity: array![
                cap[3].parse::<i64>().unwrap(),
                cap[4].parse::<i64>().unwrap()
            ],
        })
        .collect();
    println!("{:?}", part1(&robots, 103, 101));
    println!("{:?}", part2(&robots, 103, 101));
}
