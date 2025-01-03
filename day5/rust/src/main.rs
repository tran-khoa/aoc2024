use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(PartialEq, Eq, Clone)]
enum State {
    Unseen,
    Wip,
    Seen,
}

fn part1(rules: &Vec<(u32, u32)>, updates: &[Vec<u32>]) -> u32 {
    let mut adjacency_dict: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules {
        if let Some(adjacent) = adjacency_dict.get_mut(&rule.1) {
            adjacent.insert(rule.0);
        } else {
            adjacency_dict.insert(rule.1, HashSet::from([rule.0]));
        }
    }
    updates
        .iter()
        .map(|update| {
            let all_numbers: HashSet<u32> = update.iter().cloned().collect();
            let mut seen_numbers: HashSet<u32> = HashSet::new();
            for &num in update {
                if adjacency_dict.contains_key(&num)
                    && adjacency_dict[&num].iter().any(|&adj_num| {
                        all_numbers.contains(&adj_num) && !seen_numbers.contains(&adj_num)
                    })
                {
                    return 0u32;
                }
                seen_numbers.insert(num);
            }
            update[update.len() / 2]
        })
        .sum()
}

fn part2(rules: &Vec<(u32, u32)>, updates: &Vec<Vec<u32>>) -> u32 {
    let mut adjacency_dict: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules {
        if let Some(adjacent) = adjacency_dict.get_mut(&rule.0) {
            adjacent.insert(rule.1);
        } else {
            adjacency_dict.insert(rule.0, HashSet::from([rule.1]));
        }
    }

    let mut safe_middle_sum: u32 = 0;
    for update in updates {
        let mut states: Vec<State> = vec![State::Unseen; update.len()];
        let mut reorder: Vec<u32> = Vec::new();
        let unique_values: HashSet<u32> = update.iter().cloned().collect();

        let mut stack: Vec<usize> = Vec::new();

        while let Some(root_idx) = states.iter().position(|x| *x == State::Unseen) {
            stack.push(root_idx);
            while let Some(idx) = stack.pop() {
                match states[idx] {
                    State::Wip => {
                        states[idx] = State::Seen;
                        reorder.push(update[idx]);
                    }
                    State::Seen => continue,
                    State::Unseen => {
                        states[idx] = State::Wip;
                        stack.push(idx);
                        for &adj_val in adjacency_dict
                            .get(&update[idx])
                            .unwrap()
                            .intersection(&unique_values)
                        {
                            if let Some(adj_idx) = update.iter().position(|&x| x == adj_val) {
                                stack.push(adj_idx);
                            } else {
                                panic!("Value not found in update list");
                            }
                        }
                    }
                }
            }
        }
        reorder.reverse();
        if reorder != *update {
            safe_middle_sum += reorder[reorder.len() / 2];
        }
    }
    safe_middle_sum
}

fn main() {
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    match fs::read_to_string("../inputs.txt") {
        Ok(inputs) => {
            let mut update_section = false;
            for line in inputs.lines() {
                if line.is_empty() {
                    update_section = true;
                    continue;
                }
                if !update_section {
                    let rule: Vec<u32> = line.split("|").map(|num| num.parse().unwrap()).collect();
                    rules.push((rule[0], rule[1]));
                } else {
                    updates.push(line.split(",").map(|num| num.parse().unwrap()).collect());
                }
            }
        }
        Err(e) => panic!("Failed to read map.txt: {}", e),
    };
    println!("{}", part1(&rules, &updates));
    println!("{}", part2(&rules, &updates));
}
