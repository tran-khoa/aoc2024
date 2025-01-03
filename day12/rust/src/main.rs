mod grid;

use crate::grid::{Direction, Grid, PRINCIPAL_DIRECTIONS};
use std::collections::{HashMap, HashSet};

type Coords = (usize, usize);

#[derive(Clone, PartialEq, Eq)]
enum State {
    New,
    Wip,
    Seen,
}

type Plots = Grid<char>;

fn part1(plots: &Plots) -> usize {
    let mut states = Grid::new(vec![vec![State::New; plots.height]; plots.width]);
    let mut price: usize = 0;

    for root in itertools::iproduct!(0..plots.height, 0..plots.width) {
        if states[root] == State::Seen {
            continue;
        }

        let mut current_area: usize = 0;
        let mut current_perimeter: usize = 0;
        let current_label = plots[root];

        let mut stack: Vec<Coords> = Vec::new();
        stack.push(root);
        while let Some(curr) = stack.pop() {
            match states[curr] {
                State::Seen => continue,
                State::Wip => {
                    states[curr] = State::Seen;
                }
                State::New => {
                    states[curr] = State::Wip;
                    current_perimeter += 4;
                    current_area += 1;
                    PRINCIPAL_DIRECTIONS
                        .iter()
                        .filter_map(|d| plots.walk(curr, *d))
                        .for_each(|next| {
                            if plots[next] == current_label {
                                current_perimeter -= 1;
                                if states[next] == State::New {
                                    stack.push(next);
                                }
                            }
                        });
                }
            }
        }
        price += current_area * current_perimeter;
    }
    price
}

fn graph_components(plots: &Plots, vertices: &HashSet<Coords>) -> usize {
    if vertices.is_empty() {
        return 0;
    } else if vertices.len() == 1 {
        return 1;
    }
    let mut vertex_state: HashMap<Coords, State> =
        vertices.iter().map(|v| (*v, State::New)).collect();
    let mut num_components: usize = 0;
    for root in vertices {
        if vertex_state[root] != State::New {
            continue;
        }
        let mut stack: Vec<Coords> = Vec::new();
        stack.push(*root);
        num_components += 1;

        while let Some(curr_pos) = stack.pop() {
            for d in PRINCIPAL_DIRECTIONS {
                if let Some(next_pos) = plots.walk(curr_pos, d) {
                    match vertex_state.get(&next_pos) {
                        Some(State::New) => {
                            vertex_state.insert(next_pos, State::Wip);
                            stack.push(next_pos);
                        }
                        Some(State::Wip) => {
                            vertex_state.insert(next_pos, State::Seen);
                        }
                        Some(State::Seen) => continue,
                        None => continue,
                    };
                }
            }
        }
    }
    num_components
}

fn part2(plots: &Plots) -> usize {
    let mut states = Grid::new(vec![vec![State::New; plots.height]; plots.width]);
    let mut price: usize = 0;

    // outer loop: iterate over all plots
    for root in itertools::iproduct!(0..plots.height, 0..plots.width) {
        if states[root] == State::Seen {
            continue;
        }
        let current_label = plots[root];
        let mut current_area: usize = 0;
        let mut edges: HashMap<Direction, HashSet<Coords>> = HashMap::new();

        let mut stack: Vec<Coords> = Vec::new();
        stack.push(root);
        while let Some(curr_pos) = stack.pop() {
            match states[curr_pos] {
                State::Seen => continue,
                State::Wip => states[curr_pos] = State::Seen,
                State::New => {
                    states[curr_pos] = State::Wip;
                    current_area += 1;

                    for direction in PRINCIPAL_DIRECTIONS {
                        if let Some(next_pos) = plots.walk(curr_pos, direction) {
                            if plots[next_pos] == current_label {
                                if states[next_pos] == State::New {
                                    stack.push(next_pos);
                                }
                            } else {
                                let d_edges =
                                    edges.entry(direction).or_default();
                                d_edges.insert(curr_pos);
                            }
                        } else {
                            let d_edges = edges.entry(direction).or_default();
                            d_edges.insert(curr_pos);
                        }
                    }
                }
            }
        }

        let current_perimeter: usize = edges
            .values()
            .map(|d_edges| graph_components(plots, d_edges))
            .sum();
        price += current_area * current_perimeter;
    }
    price
}

fn main() {
    let plots: Vec<Vec<char>> = std::fs::read_to_string("../plots.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let plots = Plots::new(plots);

    println!("Part 1: {}", part1(&plots));
    println!("Part 2: {}", part2(&plots));
}
