use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

type PadNumDir = char;
type PadNum = char;
type Dir = char;

fn generate_adj(pad: Vec<Vec<PadNumDir>>) -> HashMap<PadNumDir, Vec<(PadNumDir, Dir)>> {
    let mut adj: HashMap<PadNumDir, Vec<(PadNumDir, Dir)>> = HashMap::new();
    for &c in pad.iter().flat_map(|row| row.iter()) {
        if c == '\0' {
            continue;
        }
        adj.insert(c, vec![]);
    }
    for (y, x) in itertools::iproduct!(0..pad.len(), 0..pad[0].len()) {
        let current = pad[y][x];
        if current == '\0' {
            continue;
        }
        for (dy, dx) in itertools::iproduct!(-1..=1, -1..=1) {
            let dir = match (dy, dx) {
                (-1, 0) => '^',
                (1, 0) => 'v',
                (0, -1) => '<',
                (0, 1) => '>',
                _ => continue,
            };
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            if ny >= 0 && ny < pad.len() as i32 && nx >= 0 && nx < pad[0].len() as i32 {
                let neighbor = pad[ny as usize][nx as usize];
                if neighbor == '\0' {
                    continue;
                }
                adj.get_mut(&current).unwrap().push((neighbor, dir));
            }
        }
    }
    adj
}
lazy_static! {
    static ref NUMPAD: Vec<Vec<char>> = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['\0', '0', 'A']
    ];
    static ref NUM_CHARS: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'];
    static ref NUMPAD_ADJ: HashMap<PadNumDir, Vec<(PadNumDir, Dir)>> = generate_adj(NUMPAD.clone());
    static ref DIRPAD: Vec<Vec<char>> = vec![vec!['\0', '^', 'A'], vec!['<', 'v', '>'],];
    static ref DIR_CHARS: Vec<char> = vec!['^', 'v', '<', '>', 'A'];
    static ref DIRPAD_ADJ: HashMap<PadNumDir, Vec<(PadNumDir, Dir)>> = generate_adj(DIRPAD.clone());
    static ref PAD_PATHS: HashMap<(PadNumDir, PadNumDir), Vec<Vec<Dir>>> = generate_all_pad_paths();
    static ref SHORTEST_PATH_PAIRS: Mutex<HashMap<(PadNum, PadNum, i32), usize>> =
        Mutex::new(HashMap::new());
    static ref SHORTEST_PATHS: Mutex<HashMap<(String, i32), Vec<Dir>>> = Mutex::new(HashMap::new());
}

fn generate_all_pad_paths() -> HashMap<(PadNum, PadNum), Vec<Vec<Dir>>> {
    // Floyd-Warshall algorithm, yes this is overkill...
    let mut distances: HashMap<(PadNum, PadNum), i32> = HashMap::new();
    let mut paths: HashMap<(PadNum, PadNum), Vec<Vec<Dir>>> = HashMap::new();

    for numpad in [true, false] {
        let chars = if numpad {
            NUM_CHARS.clone()
        } else {
            DIR_CHARS.clone()
        };
        let adj = if numpad {
            NUMPAD_ADJ.clone()
        } else {
            DIRPAD_ADJ.clone()
        };

        for (node_i, node_js) in adj.iter() {
            distances.insert((*node_i, *node_i), 0);
            paths.insert((*node_i, *node_i), vec![vec![]]);
            for (node_j, dir) in node_js {
                distances.insert((*node_i, *node_j), 1);
                paths
                    .entry((*node_i, *node_j))
                    .or_insert_with(Vec::new)
                    .push(vec![*dir]);
            }
        }
        for node_k in chars.iter() {
            for node_i in chars.iter() {
                for node_j in chars.iter() {
                    if node_i == node_j || node_i == node_k || node_j == node_k {
                        continue;
                    }
                    let &curr_d = distances.get(&(*node_i, *node_j)).unwrap_or(&i32::MAX);
                    let new_d = distances
                        .get(&(*node_i, *node_k))
                        .and_then(|&d_ik| {
                            distances.get(&(*node_k, *node_j)).map(|&d_kj| d_ik + d_kj)
                        })
                        .unwrap_or(i32::MAX);
                    if new_d < curr_d {
                        // replace current paths since taking k is shorter
                        distances.insert((*node_i, *node_j), new_d);
                        let mut new_paths = vec![];
                        for paths_ik in paths.get(&(*node_i, *node_k)).unwrap() {
                            for paths_kj in paths.get(&(*node_k, *node_j)).unwrap() {
                                let mut new_path: Vec<Dir> = vec![];
                                new_path.reserve(paths_ik.len() + paths_kj.len());
                                new_path.extend(paths_ik);
                                new_path.extend(paths_kj);
                                new_paths.push(new_path);
                            }
                        }
                        paths.insert((*node_i, *node_j), new_paths);
                    } else if new_d == curr_d {
                        if new_d == i32::MAX {
                            continue;
                        }
                        // add new paths since taking k is equal
                        let mut new_paths: Vec<Vec<Dir>> = vec![];
                        for paths_ik in paths.get(&(*node_i, *node_k)).unwrap() {
                            for paths_kj in paths.get(&(*node_k, *node_j)).unwrap() {
                                let mut new_path: Vec<Dir> = vec![];
                                new_path.reserve(paths_ik.len() + paths_kj.len());
                                new_path.extend(paths_ik);
                                new_path.extend(paths_kj);
                                new_paths.push(new_path);
                            }
                        }
                        paths
                            .get_mut(&(*node_i, *node_j))
                            .get_or_insert(&mut vec![])
                            .extend(new_paths);
                    }
                }
            }
        }
    }
    paths
}

fn shortest_path_pair(a: PadNum, b: PadNum, level: i32) -> usize {
    /*
    Assuming we are currently at char `a` at level `level + 1`,
    how many buttons does level `1` have to press in order to press b at the current level?
     */
    {
        let cache = SHORTEST_PATH_PAIRS.lock().unwrap();
        if cache.contains_key(&(a, b, level)) {
            return cache.get(&(a, b, level)).unwrap().clone();
        }
    }
    let paths: Vec<Vec<char>> = PAD_PATHS
        .get(&(a, b))
        .unwrap() // get from a to b first
        .iter()
        .map(|path_ab| {
            let mut path_ab = path_ab.clone();
            path_ab.push('A');
            path_ab
        }) // then confirm selection
        .collect();

    let final_path = if level > 1 {
        paths
            .iter()
            .map(|path| shortest_path(path.clone(), level - 1))
            .min()
            .unwrap()
    } else {
        assert_eq!(level, 1);
        paths.iter().map(|path| path.len()).min().unwrap()
    };

    let mut cache = SHORTEST_PATH_PAIRS.lock().unwrap();
    cache.insert((a, b, level), final_path.clone());

    final_path.clone()
}

fn shortest_path(sequence: Vec<char>, levels: i32) -> usize {
    std::iter::once('A')
        .chain(sequence.clone().into_iter())
        .tuple_windows()
        .map(|(a, b)| shortest_path_pair(a, b, levels))
        .sum()
}

fn run(inputs: &Vec<(Vec<char>, usize)>, part1: bool) -> usize {
    let levels = if part1 { 3 } else { 26 };
    inputs
        .iter()
        .map(|(sequence, number)| {
            let path = shortest_path(sequence.clone(), levels);
            path * number
        })
        .sum()
}

fn main() {
    let inputs: Vec<(Vec<char>, usize)> = std::fs::read_to_string("../inputs.txt")
        .map(|file| {
            file.lines()
                .map(|line| {
                    (
                        line.chars().collect(),
                        line[..line.len() - 1].parse::<usize>().unwrap(),
                    )
                })
                .collect()
        })
        .unwrap();
    println!("Part 1: {}", run(&inputs, true));
    println!("Part 2: {}", run(&inputs, false));
}
