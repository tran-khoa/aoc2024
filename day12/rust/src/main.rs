use std::ops::Deref;

#[derive(Clone, PartialEq, Eq)]
enum State {
    NEW,
    WIP,
    SEEN,
}

#[derive(Clone, PartialEq, Eq, Copy)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

struct Plots {
    data: Vec<Vec<char>>,
    height: usize,
    width: usize,
}
impl Plots {
    fn new(data: Vec<Vec<char>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        Plots {
            data,
            height,
            width,
        }
    }
    fn valid_coords(&self, xy: (i32, i32)) -> bool {
        0 <= xy.0 && xy.0 < self.height as i32 && 0 <= xy.1 && xy.1 < self.width as i32
    }
    fn walk(&self, pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let pos: (i32, i32) = (pos.0 as i32, pos.1 as i32);
        let new_pos = match direction {
            Direction::RIGHT => (pos.0, pos.1 + 1),
            Direction::DOWN => (pos.0 + 1, pos.1),
            Direction::LEFT => (pos.0, pos.1 - 1),
            Direction::UP => (pos.0 - 1, pos.1),
        };
        if self.valid_coords(new_pos) {
            Some((new_pos.0 as usize, new_pos.1 as usize))
        } else {
            None
        }
    }
    fn get_label(&self, pos: (usize, usize)) -> char {
        self.data[pos.0][pos.1]
    }
    fn follow_edge(
        &self,
        pos: (usize, usize),
        current_direction: Direction,
        current_label: char,
    ) -> ((usize, usize), Direction) {
        let next_pos = self.walk(pos, current_direction);
        if next_pos.is_some() && self.get_label(next_pos.unwrap()) == current_label {
            (next_pos.unwrap(), current_direction)
        } else {
            let next_direction = match current_direction {
                Direction::RIGHT => Direction::DOWN,
                Direction::DOWN => Direction::LEFT,
                Direction::LEFT => Direction::UP,
                Direction::UP => Direction::RIGHT,
            };
            (self.walk(pos, next_direction).unwrap(), next_direction)
        }
    }
}
impl Deref for Plots {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn part1(plots: &Plots) -> usize {
    let mut states = vec![vec![State::NEW; plots.height]; plots.width];
    let mut price: usize = 0;

    for (i, j) in itertools::iproduct!(0..plots.height, 0..plots.width) {
        if states[i][j] == State::SEEN {
            continue;
        }

        let mut current_area: usize = 0;
        let mut current_perimeter: usize = 0;
        let current_label = plots[i][j];

        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((i, j));
        while stack.len() > 0 {
            let (x, y) = stack.pop().unwrap();
            match states[x][y] {
                State::SEEN => continue,
                State::WIP => {
                    states[x][y] = State::SEEN;
                }
                State::NEW => {
                    states[x][y] = State::WIP;
                    current_perimeter += 4;
                    current_area += 1;
                    [
                        Direction::DOWN,
                        Direction::RIGHT,
                        Direction::UP,
                        Direction::LEFT,
                    ]
                    .iter()
                    .filter_map(|new_pos| plots.walk((x, y), *new_pos))
                    .for_each(|(nx, ny)| {
                        if plots[nx][ny] == current_label {
                            current_perimeter -= 1;
                            if states[nx][ny] == State::NEW {
                                stack.push((nx, ny));
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

fn compute_perimeter(plots: &Plots, start: (usize, usize)) -> usize {
    let label = plots.get_label(start);

    let mut perimeter = 0;
    for dir in [
        Direction::DOWN,
        Direction::RIGHT,
        Direction::UP,
        Direction::LEFT,
    ] {
        if let Some(next_pos) = plots.walk(pos, dir) {
            if plots[next_pos.0][next_pos.1] != label {
                perimeter += 1;
            }
        }
    }
    perimeter
}

fn part2(plots: &Plots) -> usize {
    // invariant: for each dfs call, the root is an 'upper left' edge corner
    let mut states = vec![vec![State::NEW; plots.height]; plots.width];
    let mut price: usize = 0;
    for root in itertools::iproduct!(0..plots.height, 0..plots.width) {
        if states[root.0][root.1] == State::SEEN {
            continue;
        }
        let mut current_area: usize = 0;
        let mut current_perimeter: usize = 0;
        let current_label = plots[root.0][root.1];
        let mut current_edge_direction = Some(Direction::RIGHT);

        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push(root);
        while stack.len() > 0 {
            let (i, j) = stack.pop().unwrap();
            match states[i][j] {
                State::SEEN => continue,
                State::WIP => states[i][j] = State::SEEN,
                State::NEW => {
                    states[i][j] = State::WIP;
                    current_area += 1;

                    if let Some(edge_direction) = current_edge_direction {
                        let (next_pos, next_direction) = plots.follow_edge((i, j), edge_direction);
                        if edge_direction != next_direction {
                            current_perimeter += 1;
                        }
                        if next_pos == root {
                            current_edge_direction = None;
                        } else {
                            current_edge_direction = Some(next_direction);
                            for dir in [
                                Direction::DOWN,
                                Direction::RIGHT,
                                Direction::UP,
                                Direction::LEFT,
                            ] {
                                if dir != next_direction {
                                    if let Some(next_pos) = plots.walk(next_pos, dir) {
                                        if plots[next_pos.0][next_pos.1] == current_label {
                                            stack.push(next_pos);
                                        }
                                    }
                                }
                            }
                            stack.push(next_pos); // push last
                        }
                    } else {
                        [
                            Direction::DOWN,
                            Direction::RIGHT,
                            Direction::UP,
                            Direction::LEFT,
                        ]
                        .iter()
                        .filter_map(|new_pos| plots.walk((i, j), *new_pos))
                        .for_each(|(nx, ny)| {
                            if plots[nx][ny] == current_label {
                                if states[nx][ny] == State::NEW {
                                    stack.push((nx, ny));
                                }
                            }
                        });
                    }
                }
            }
        }
        price += current_area * current_perimeter;
        println!(
            "Plot {}, area={}, perimeter={}",
            current_label, current_area, current_perimeter
        );
    }
    price
}

fn main() {
    let plots: Vec<Vec<char>> = std::fs::read_to_string("../test_plots.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let plots = Plots::new(plots);

    println!("Part 1: {}", part1(&plots));
    println!("Part 2: {}", part2(&plots));
}
