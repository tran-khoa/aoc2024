#[derive(Debug, Clone)]
struct Equation {
    target: u64,
    values: Vec<u64>
}

fn is_evaluable(eq: Equation, with_concat: bool) -> bool {
    if eq.values.len() == 1 {
        return eq.target == eq.values[0];
    }
    let merges = if with_concat {vec![
        eq.values[0] + eq.values[1],
        eq.values[0] * eq.values[1],
        (eq.values[0].to_string() + &eq.values[1].to_string()).parse().unwrap()
    ]} else {vec![
        eq.values[0] + eq.values[1],
        eq.values[0] * eq.values[1]
    ]};
    merges.iter().any(|merge| {
        if *merge > eq.target {
            return false;
        }
        let mut new_values = vec![*merge];
        new_values.extend_from_slice(&eq.values[2..]);
        return is_evaluable(Equation {
            target: eq.target,
            values: new_values
        }, with_concat);
    })
}

fn part1(equations: &Vec<Equation>) -> u64 {
    equations.iter().map(|eq| {
        if is_evaluable(eq.clone(), false) {
            return eq.target;
        }
        return 0;
    }).sum()
}

fn part2(equations: &Vec<Equation>) -> u64 {
    equations.iter().map(|eq| {
        if is_evaluable(eq.clone(), true) {
            return eq.target;
        }
        return 0;
    }).sum()
}

fn main() {
    let equations: Vec<Equation> = match std::fs::read_to_string("../equations.txt") {
        Ok(content) => {
            content.lines().map(|line| {
                let parts: Vec<&str> = line.split(": ").collect();
                assert_eq!(parts.len(), 2);
                Equation {
                    target: parts[0].parse().unwrap(),
                    values: parts[1].split(" ").map(|num| num.parse().unwrap()).collect()
                }
            }).collect()
        },
        Err(_) => {
            println!("Error reading file");
            return;
        }
    };
    println!("Part 1: {}", part1(&equations));
    println!("Part 2: {}", part2(&equations));
}
