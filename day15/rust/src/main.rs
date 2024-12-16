use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Entity {
    Wall,
    Robot,
    Box,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity2 {
    Wall,
    Robot,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn translate(&self, robot: &Coords) -> Coords {
        match self {
            Direction::UP => (robot.0 - 1, robot.1),
            Direction::DOWN => (robot.0 + 1, robot.1),
            Direction::LEFT => (robot.0, robot.1 - 1),
            Direction::RIGHT => (robot.0, robot.1 + 1),
        }
    }
}

type Coords = (i32, i32);
type Map = HashMap<Coords, Entity>;
type Map2 = HashMap<Coords, Entity2>;

fn map_move<T: Copy>(map: &mut HashMap<Coords, T>, coords: &Coords, dir: &Direction) {
    let entity = map[coords];
    let new_coords = dir.translate(coords);
    map.remove(&coords);
    map.insert(new_coords, entity);
}

fn apply_map_moves<T: Copy>(
    map: &mut HashMap<Coords, T>,
    coords: &HashSet<Coords>,
    dir: &Direction,
) {
    let new_coords: Vec<(Coords, T)> = coords.iter().map(|c| (dir.translate(c), map[c])).collect();
    coords.iter().for_each(|c| {
        map.remove(c);
    });
    new_coords.iter().for_each(|(c, e)| {
        map.insert(*c, *e);
    });
}

fn move_box(map: &mut Map, box_coords: &Coords, dir: &Direction) -> bool {
    let next = dir.translate(&box_coords);

    match map.get(&next) {
        Some(Entity::Wall) => false,
        Some(Entity::Robot) => panic!("how even"),
        None => {
            map_move(map, &box_coords, dir);
            true
        }
        Some(Entity::Box) => {
            if move_box(map, &next, &dir) {
                map_move(map, &box_coords, dir);
                return true;
            }
            false
        }
    }
}

fn part1(map_str: &str, dirs: &Vec<Direction>) -> i32 {
    let mut map: Map = HashMap::new();
    let mut robot: Option<Coords> = None;
    map_str.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            let coords = (row as i32, col as i32);
            let entity = match c {
                '#' => Entity::Wall,
                'O' => Entity::Box,
                '@' => {
                    robot = Some(coords);
                    Entity::Robot
                }
                '.' => return,
                _ => {
                    panic!("Invalid character '{}'", c);
                }
            };
            map.insert(coords, entity);
        })
    });
    let mut robot = robot.unwrap();

    for m in dirs {
        let next_coord = m.translate(&robot);
        match map.get(&next_coord) {
            Some(Entity::Wall) => continue,
            None => {
                map_move(&mut map, &robot, m);
                robot = next_coord
            }
            Some(Entity::Robot) => panic!("how even"),
            Some(Entity::Box) => {
                if move_box(&mut map, &next_coord, &m) {
                    map_move(&mut map, &robot, m);
                    robot = next_coord;
                }
            }
        }
    }
    map.iter()
        .filter_map(|(&coord, entity)| match entity {
            Entity::Box => Some(100 * coord.0 + coord.1),
            _ => None,
        })
        .sum()
}

fn try_move_large(map: &Map2, coords: &Coords, dir: &Direction) -> Option<HashSet<Coords>> {
    match map.get(coords) {
        None => Some(HashSet::new()),
        Some(Entity2::Wall) => None,
        Some(Entity2::Robot) => match try_move_large(map, &dir.translate(coords), dir) {
            Some(mut ts) => {
                ts.insert(*coords);
                Some(ts)
            }
            None => None,
        },
        Some(large_box) => {
            let (box_left, box_right) = match large_box {
                Entity2::BoxLeft => (*coords, Direction::RIGHT.translate(&coords)),
                Entity2::BoxRight => (Direction::LEFT.translate(&coords), *coords),
                _ => panic!("can't happen"),
            };
            match dir {
                Direction::LEFT => {
                    let next = dir.translate(&box_left);
                    match try_move_large(map, &next, dir) {
                        None => None,
                        Some(mut ts) => {
                            ts.insert(box_left);
                            ts.insert(box_right);
                            Some(ts)
                        }
                    }
                }
                Direction::RIGHT => {
                    let next = dir.translate(&box_right);
                    match try_move_large(map, &next, dir) {
                        None => None,
                        Some(mut ts) => {
                            ts.insert(box_right);
                            ts.insert(box_left);
                            Some(ts)
                        }
                    }
                }
                Direction::UP | Direction::DOWN => {
                    let next_left = dir.translate(&box_left);
                    let next_right = dir.translate(&box_right);

                    let mut ts_left = match try_move_large(map, &next_left, dir) {
                        None => return None,
                        Some(ts) => ts,
                    };
                    if map
                        .get(&next_left)
                        .map(|&e| e == Entity2::BoxLeft)
                        .unwrap_or(false)
                    {
                        // another box perfectly aligned above/below, no need to check right
                        ts_left.insert(box_left);
                        ts_left.insert(box_right);
                        return Some(ts_left);
                    }
                    match try_move_large(map, &next_right, dir) {
                        None => None,
                        Some(ts) => {
                            let mut ts: HashSet<Coords> =
                                ts.union(&mut ts_left).map(|&x| x).collect();
                            ts.insert(box_left);
                            ts.insert(box_right);
                            Some(ts)
                        }
                    }
                }
            }
        }
    }
}

fn part2(map_str: &str, directions: &Vec<Direction>) -> i32 {
    let mut map: Map2 = HashMap::new();
    let mut robot: Option<Coords> = None;
    map_str.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(col, c)| {
            let left_coord = (row as i32, col as i32 * 2);
            let right_coord = (row as i32, col as i32 * 2 + 1);
            let entity = match c {
                '#' => (Entity2::Wall, Some(Entity2::Wall)),
                'O' => (Entity2::BoxLeft, Some(Entity2::BoxRight)),
                '@' => {
                    robot = Some(left_coord);
                    (Entity2::Robot, None)
                }
                '.' => return,
                _ => {
                    panic!("Invalid character '{}'", c);
                }
            };
            map.insert(left_coord, entity.0);
            if let Some(e) = entity.1 {
                map.insert(right_coord, e);
            }
        })
    });
    let map_height = map_str.lines().count() as i32;
    let map_width = map_str.lines().next().unwrap().len() as i32 * 2;
    let mut robot = robot.unwrap();
    for d in directions {
        let move_coords = try_move_large(&map, &robot, d);
        if let Some(ts) = move_coords {
            apply_map_moves(&mut map, &ts, d);
            robot = d.translate(&robot);
        }
    }
    print_map2(&map, map_height, map_width);
    map.iter()
        .filter_map(|(&coord, entity)| match entity {
            Entity2::BoxLeft => Some(100 * coord.0 + coord.1),
            _ => None,
        })
        .sum()
}

fn print_map2(map2: &Map2, height: i32, width: i32) {
    for y in 0..height {
        for x in 0..width {
            print!(
                "{}",
                match map2.get(&(y, x)) {
                    None => '.',
                    Some(Entity2::BoxLeft) => '[',
                    Some(Entity2::BoxRight) => ']',
                    Some(Entity2::Wall) => '#',
                    Some(Entity2::Robot) => '@',
                }
            )
        }
        print!("\n");
    }
}

fn main() {
    let input_str = std::fs::read_to_string("../inputs.txt").unwrap();
    let input_vec: Vec<&str> = input_str.split("\n\n").collect();
    let dirs: Vec<Direction> = input_vec[1]
        .lines()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => Direction::UP,
                'v' => Direction::DOWN,
                '<' => Direction::LEFT,
                '>' => Direction::RIGHT,
                _ => panic!("Invalid character '{}'", c),
            })
        })
        .collect();
    println!("Part 1: {:?}", part1(input_vec[0], &dirs));
    println!("Part 2: {:?}", part2(input_vec[0], &dirs));
}
