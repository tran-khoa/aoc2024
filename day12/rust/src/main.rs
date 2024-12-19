mod grid;

use crate::grid::{Direction, Grid, PRINCIPAL_DIRECTIONS};
use std::collections::{HashMap, HashSet};

type Coords = (usize, usize);

#[derive(Clone, PartialEq, Eq)]
enum State {
    NEW,
    WIP,
    SEEN,
}

type Plots = Grid<char>;

fn part1(plots: &Plots) -> usize {
    let mut states = Grid::new(vec![vec![State::NEW; plots.height]; plots.width]);
    let mut price: usize = 0;

    for root in itertools::iproduct!(0..plots.height, 0..plots.width) {
        if states[root] == State::SEEN {
            continue;
        }

        let mut current_area: usize = 0;
        let mut current_perimeter: usize = 0;
        let current_label = plots[root];

        let mut stack: Vec<Coords> = Vec::new();
        stack.push(root);
        while stack.len() > 0 {
            let curr = stack.pop().unwrap();
            match states[curr] {
                State::SEEN => continue,
                State::WIP => {
                    states[curr] = State::SEEN;
                }
                State::NEW => {
                    states[curr] = State::WIP;
                    current_perimeter += 4;
                    current_area += 1;
                    PRINCIPAL_DIRECTIONS
                        .iter()
                        .filter_map(|d| plots.walk(curr, *d))
                        .for_each(|next| {
                            if plots[next] == current_label {
                                current_perimeter -= 1;
                                if states[next] == State::NEW {
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
    if vertices.len() == 0 {
        return 0;
    } else if vertices.len() == 1 {
        return 1;
    }
    let mut vertex_state: HashMap<Coords, State> =
        vertices.iter().map(|v| (*v, State::NEW)).collect();
    let mut num_components: usize = 0;
    for root in vertices {
        if vertex_state[root] != State::NEW {
            continue;
        }
        let mut stack: Vec<Coords> = Vec::new();
        stack.push(*root);
        num_components += 1;

        while stack.len() > 0 {
            let curr_pos = stack.pop().unwrap();
            for d in PRINCIPAL_DIRECTIONS {
                if let Some(next_pos) = plots.walk(curr_pos, d) {
                    match vertex_state.get(&next_pos) {
                        Some(State::NEW) => {
                            vertex_state.insert(next_pos, State::WIP);
                            stack.push(next_pos);
                        }
                        Some(State::WIP) => {
                            vertex_state.insert(next_pos, State::SEEN);
                        }
                        Some(State::SEEN) => continue,
                        None => continue,
                    };
                }
            }
        }
    }
    num_components
}

fn part2(plots: &Plots) -> usize {
    let mut states = Grid::new(vec![vec![State::NEW; plots.height]; plots.width]);
    let mut price: usize = 0;

    // outer loop: iterate over all plots
    for root in itertools::iproduct!(0..plots.height, 0..plots.width) {
        if states[root] == State::SEEN {
            continue;
        }
        let current_label = plots[root];
        let mut current_area: usize = 0;
        let mut edges: HashMap<Direction, HashSet<Coords>> = HashMap::new();

        let mut stack: Vec<Coords> = Vec::new();
        stack.push(root);
        while stack.len() > 0 {
            let curr_pos = stack.pop().unwrap();
            match states[curr_pos] {
                State::SEEN => continue,
                State::WIP => states[curr_pos] = State::SEEN,
                State::NEW => {
                    states[curr_pos] = State::WIP;
                    current_area += 1;

                    for direction in PRINCIPAL_DIRECTIONS {
                        if let Some(next_pos) = plots.walk(curr_pos, direction) {
                            if plots[next_pos] == current_label {
                                if states[next_pos] == State::NEW {
                                    stack.push(next_pos);
                                }
                            } else {
                                let d_edges =
                                    edges.entry(direction).or_insert_with(|| HashSet::new());
                                d_edges.insert(curr_pos);
                            }
                        } else {
                            let d_edges = edges.entry(direction).or_insert_with(|| HashSet::new());
                            d_edges.insert(curr_pos);
                        }
                    }
                }
            }
        }

        let current_perimeter: usize = edges
            .values()
            .map(|d_edges| graph_components(plots, &d_edges))
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
