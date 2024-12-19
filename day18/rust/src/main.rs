use std::collections::{HashMap, HashSet, VecDeque};

type Coords = (i32, i32);

fn get_inputs(path: &str) -> Vec<Coords> {
    let contents = std::fs::read_to_string(path).unwrap();
    let coords: Vec<Coords> = contents
        .lines()
        .map(|s| {
            let mut parts = s.split(",");
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    coords
}

fn get_adjacent(obstacles: &[Coords], of: &Coords, max_x: i32, max_y: i32) -> Vec<Coords> {
    let mut adjacents: Vec<Coords> = Vec::new();
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
        let x = of.0 + dx;
        let y = of.1 + dy;
        if x >= 0 && y >= 0 && x <= max_x && y <= max_y && !obstacles.contains(&(x, y)) {
            adjacents.push((x, y));
        }
    }
    adjacents
}

fn part1(obstacles: &Vec<Coords>, max_x: i32, max_y: i32, first_bytes: usize) -> i32 {
    let first_kb = &obstacles[0..first_bytes];
    let mut explored: HashSet<Coords> = HashSet::new();
    let mut queue: VecDeque<Coords> = VecDeque::new();
    let mut parents: HashMap<Coords, Coords> = HashMap::new();
    let root: Coords = (0, 0);
    explored.insert(root);
    queue.push_back(root);

    let mut found_path: bool = false;
    while queue.len() > 0 {
        let node = queue.pop_front().unwrap();
        if node == (max_x, max_y) {
            found_path = true;
            break;
        }
        for neighbor in get_adjacent(first_kb, &node, max_x, max_y) {
            if !explored.contains(&neighbor) {
                explored.insert(neighbor);
                parents.insert(neighbor, node);
                queue.push_back(neighbor);
            }
        }
    }
    if found_path {
        let mut path_length = 0;
        let mut current_node = (max_x, max_y);
        while current_node != root {
            path_length += 1;
            current_node = parents[&current_node];
        }
        path_length
    } else {
        0
    }
}

fn part2(obstacles: &Vec<Coords>, max_x: i32, max_y: i32) -> Coords {
    let mut left = 0;
    let mut right = obstacles.len();
    while left < right {
        let mid = (left + right) / 2;
        let path_length = part1(obstacles, max_x, max_y, mid);
        if path_length > 0 {
            // still a path possible, got go further
            left = mid + 1;
        } else {
            // path not possible, but can we find a better one
            right = mid;
        }
    }
    obstacles[left - 1]
}

fn main() {
    let example = get_inputs("../example_coords.txt");
    let real_inputs = get_inputs("../coords.txt");
    println!("Part 1 (Example): {}", part1(&example, 6, 6, 12));
    println!("Part 1 (Real): {}", part1(&real_inputs, 70, 70, 1024));
    println!("Part 2 (Example): {:?}", part2(&example, 6, 6));
    println!("Part 2 (Real): {:?}", part2(&real_inputs, 70, 70));
}
