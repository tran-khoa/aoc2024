use std::ops::{Index, IndexMut};

pub type Coords = (usize, usize);
#[derive(Clone, PartialEq, Eq, Copy, Hash, Debug)]
pub enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

pub const PRINCIPAL_DIRECTIONS: [Direction; 4] = [
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
    Direction::UP,
];

pub struct Grid<T> {
    data: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}
impl<T> Index<Coords> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coords) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl<T> IndexMut<Coords> for Grid<T> {
    fn index_mut(&mut self, index: Coords) -> &mut T {
        &mut self.data[index.0][index.1]
    }
}
impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        Grid {
            data,
            height,
            width,
        }
    }
    pub fn valid_coords(&self, xy: (i32, i32)) -> bool {
        0 <= xy.0 && xy.0 < self.height as i32 && 0 <= xy.1 && xy.1 < self.width as i32
    }
    pub fn walk(&self, pos: Coords, direction: Direction) -> Option<Coords> {
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
}
