use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

type Coords = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn translate(&self, from: &Coords) -> Coords {
        match self {
            Direction::UP => (from.0 - 1, from.1),
            Direction::DOWN => (from.0 + 1, from.1),
            Direction::LEFT => (from.0, from.1 - 1),
            Direction::RIGHT => (from.0, from.1 + 1),
        }
    }
    fn turn_clockwise(&self) -> Direction {
        match &self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
    fn turn_counter_clockwise(&self) -> Direction {
        match &self {
            Direction::UP => Direction::LEFT,
            Direction::RIGHT => Direction::UP,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::DOWN,
        }
    }
}

fn parse_maze(maze_str: &str) -> (HashSet<Coords>, Coords, Coords) {
    let mut walls: HashSet<Coords> = HashSet::new();
    let mut start: Option<Coords> = None;
    let mut end: Option<Coords> = None;
    for (y, line) in maze_str.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((y as i32, x as i32));
                }
                'S' => {
                    start = Some((y as i32, x as i32));
                }
                'E' => {
                    end = Some((y as i32, x as i32));
                }
                _ => continue,
            }
        }
    }
    (walls, start.unwrap(), end.unwrap())
}

fn penalized_l1_distance(a: Coords, b: Coords) -> i32 {
    let diff_x = (a.1 - b.1).abs();
    let diff_y = (a.0 - b.0).abs();
    if diff_x > 0 || diff_y > 0 {
        diff_x + diff_y + 3
    } else {
        diff_x + diff_y
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Node {
    coords: Coords,
    direction: Direction,
}
impl Node {
    fn follow(&self) -> Node {
        Node {
            coords: self.direction.translate(&self.coords),
            direction: self.direction,
        }
    }
    fn turn_clockwise(&self) -> Node {
        Node {
            coords: self.coords,
            direction: self.direction.turn_clockwise(),
        }
    }
    fn turn_counter_clockwise(&self) -> Node {
        Node {
            coords: self.coords,
            direction: self.direction.turn_counter_clockwise(),
        }
    }
}

#[derive(Copy, Clone)]
struct FScoreNode {
    f_score: i32,
    node: Node,
}
impl Eq for FScoreNode {}
impl PartialEq<Self> for FScoreNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}
impl PartialOrd<Self> for FScoreNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.f_score.cmp(&other.f_score).reverse())
    }
}
impl Ord for FScoreNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

struct HashHeap {
    hash_set: HashSet<Node>,
    binary_heap: BinaryHeap<FScoreNode>,
}

impl HashHeap {
    fn new() -> Self {
        HashHeap {
            hash_set: HashSet::new(),
            binary_heap: BinaryHeap::new(),
        }
    }
    fn insert(&mut self, node: Node, score: i32) {
        if self.hash_set.contains(&node) {
            // could be done in O(log(n)) by binary search, but not allowed by rust...
            // unfortunately O(n+log(n))
            self.binary_heap.retain(|x| x.node != node)
        }
        self.binary_heap.push(FScoreNode {
            f_score: score,
            node,
        });
        self.hash_set.insert(node);
    }
    fn pop(&mut self) -> Option<FScoreNode> {
        // removes value from heap but not from hashmap
        let value = self.binary_heap.pop();
        if let Some(fnode) = value {
            self.hash_set.remove(&fnode.node);
        }
        value
    }
    fn is_empty(&self) -> bool {
        self.hash_set.is_empty()
    }
}

fn part1(walls: &HashSet<Coords>, start_xy: Coords, end_xy: Coords) -> Option<i32> {
    let mut open_set: HashHeap = HashHeap::new();
    let mut f_scores: HashMap<Node, i32> = HashMap::new();
    let mut g_scores: HashMap<Node, i32> = HashMap::new();
    let start_node = Node {
        coords: start_xy,
        direction: Direction::RIGHT,
    };
    open_set.insert(start_node, penalized_l1_distance(start_xy, end_xy));
    f_scores.insert(start_node, penalized_l1_distance(start_xy, end_xy));
    g_scores.insert(start_node, 0);

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap();
        if current.node.coords == end_xy {
            return Some(g_scores[&current.node]);
        }
        // follow current direction
        let follow_neighbor = current.node.follow();
        if !walls.contains(&follow_neighbor.coords) {
            let tentative_g_score = g_scores[&current.node] + 1;
            if g_scores
                .get(&follow_neighbor)
                .map(|neighbor_g| tentative_g_score < *neighbor_g)
                .unwrap_or(true)
            {
                g_scores.insert(follow_neighbor, tentative_g_score);
                let f_score =
                    tentative_g_score + penalized_l1_distance(follow_neighbor.coords, end_xy);
                f_scores.insert(follow_neighbor, f_score);
                open_set.insert(follow_neighbor, f_score);
            }
        }
        // turn left/right
        let turn_neighbors = [
            current.node.turn_clockwise(),
            current.node.turn_counter_clockwise(),
        ];
        for n in turn_neighbors {
            let tentative_g_score = g_scores[&current.node] + 1000;
            if g_scores
                .get(&n)
                .map(|neighbor_g| tentative_g_score < *neighbor_g)
                .unwrap_or(true)
            {
                g_scores.insert(n, tentative_g_score);
                let f_score = tentative_g_score + penalized_l1_distance(n.coords, end_xy);
                f_scores.insert(n, f_score);
                open_set.insert(n, f_score);
            }
        }
    }
    None
}

fn part2(walls: &HashSet<Coords>, start_xy: Coords, end_xy: Coords) -> Option<i32> {
    let mut open_set: HashHeap = HashHeap::new();
    let mut f_scores: HashMap<Node, i32> = HashMap::new();
    let start_node = Node {
        coords: start_xy,
        direction: Direction::RIGHT,
    };
    open_set.insert(start_node, penalized_l1_distance(start_xy, end_xy));
    f_scores.insert(start_node, penalized_l1_distance(start_xy, end_xy));

    let mut came_from: HashMap<Node, HashSet<Node>> = HashMap::new();

    let mut g_scores: HashMap<Node, i32> = HashMap::new();
    g_scores.insert(start_node, 0);

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap();
        if current.node.coords == end_xy {
            continue; // first change: continue search
        }
        // follow current direction
        let follow_n = current.node.follow();
        if !walls.contains(&follow_n.coords) {
            let tentative_g_score = g_scores[&current.node] + 1;
            if g_scores
                .get(&follow_n)
                .map(|neighbor_g| tentative_g_score < *neighbor_g)
                .unwrap_or(true)
            {
                came_from.insert(follow_n, HashSet::from([current.node]));
                g_scores.insert(follow_n, tentative_g_score);
                let f_score = tentative_g_score + penalized_l1_distance(follow_n.coords, end_xy);
                f_scores.insert(follow_n, f_score);
                open_set.insert(follow_n, f_score);
            } else if g_scores
                .get(&follow_n)
                .map(|&neighbor_g| tentative_g_score == neighbor_g)
                .unwrap_or(false)
            {
                came_from
                    .get_mut(&follow_n)
                    .get_or_insert(&mut HashSet::new())
                    .insert(current.node);
            }
        }
        // turn left/right
        let turn_neighbors = [
            current.node.turn_clockwise(),
            current.node.turn_counter_clockwise(),
        ];
        for n in turn_neighbors {
            let tentative_g_score = g_scores[&current.node] + 1000;
            if g_scores
                .get(&n)
                .map(|neighbor_g| tentative_g_score < *neighbor_g)
                .unwrap_or(true)
            {
                came_from.insert(n, HashSet::from([current.node]));
                g_scores.insert(n, tentative_g_score);
                let f_score = tentative_g_score + penalized_l1_distance(n.coords, end_xy);
                f_scores.insert(n, f_score);
                open_set.insert(n, f_score);
            } else if g_scores
                .get(&n)
                .map(|&neighbor_g| tentative_g_score == neighbor_g)
                .unwrap_or(false)
            {
                came_from
                    .get_mut(&n)
                    .get_or_insert(&mut HashSet::new())
                    .insert(current.node);
            }
        }
    }

    let mut todo_set: Vec<&Node> = came_from
        .iter()
        .filter(|(&k, _)| k.coords == end_xy)
        .min_by_key(|(&k, _)| g_scores[&k])
        .unwrap()
        .1
        .iter()
        .collect();
    let mut visited_set: HashSet<Coords> = HashSet::new();
    visited_set.insert(end_xy);
    while todo_set.len() > 0 {
        let curr_node = todo_set.pop().unwrap();
        visited_set.insert(curr_node.coords);
        if curr_node.coords != start_xy {
            came_from[curr_node].iter().for_each(|n| todo_set.push(&n));
        }
    }

    // col = 14, row = 15
    for r in 0..15 {
        for col in 0..15 {
            if walls.contains(&(r, col)) {
                print!("#");
            } else {
                if visited_set.contains(&(r, col)) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
        }
        print!("\n");
    }

    Some(visited_set.len() as i32)
}

fn main() {
    let maze_str = std::fs::read_to_string("../maze.txt").unwrap();
    let (walls, start, end) = parse_maze(&maze_str);
    println!("Part 1: {}", part1(&walls, start, end).unwrap());
    println!("Part 2: {}", part2(&walls, start, end).unwrap());
}
