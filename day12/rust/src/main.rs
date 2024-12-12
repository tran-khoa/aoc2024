use std::ops::Deref;

#[derive(Clone, PartialEq, Eq)]
enum State {
    NEW,
    WIP,
    SEEN,
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
    fn neighbor_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as i32;
        let y = y as i32;
        [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .iter()
            .filter(|(x, y)| {
                0 <= *x && *x < self.height as i32 && 0 <= *y && *y < self.width as i32
            })
            .map(|(x, y)| (*x as usize, *y as usize))
            .collect()
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
    let mut current_label: char;
    let mut current_area: usize = 0;
    let mut current_perimeter: usize = 0;

    for (i, j) in itertools::iproduct!(0..plots.height, 0..plots.width) {
        if states[i][j] == State::SEEN {
            continue;
        }
        current_label = plots[i][j];
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
                    for (nx, ny) in plots.neighbor_coords(x, y) {
                        if plots[nx][ny] == current_label {
                            current_perimeter -= 1;
                            if states[nx][ny] == State::NEW {
                                stack.push((nx, ny));
                            }
                        }
                    }
                }
            }
        }

        price += current_area * current_perimeter;
        current_area = 0;
        current_perimeter = 0;
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
}
