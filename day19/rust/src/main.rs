use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;

fn parse_inputs(input_str: &String) -> (Vec<&str>, Vec<&str>) {
    let parts: Vec<_> = input_str.split("\n\n").collect();
    let patterns: Vec<_> = parts[0].split(", ").collect();
    let designs: Vec<_> = parts[1].split("\n").collect();
    (patterns, designs)
}

fn part1(patterns: &Vec<&str>, designs: &Vec<&str>) -> usize {
    let regex_str = format!("^({})*$", patterns.join("|"));
    let regex = Regex::new(&*regex_str).unwrap();
    designs.iter().filter(|d| regex.is_match(*d)).count()
}

struct PatternsUntilIndex {
    patterns: Vec<String>,
    design: String,
    cache: RefCell<HashMap<usize, usize>>,
}
impl PatternsUntilIndex {
    fn new(patterns: &Vec<&str>, design: &str) -> Self {
        let mut cache: HashMap<usize, usize> = HashMap::new();
        cache.insert(0,
                     patterns.iter().any(|p| p.len() == 1 && design.starts_with(p)) as usize);
        Self {
            patterns: patterns.iter().map(|p| p.to_string()).collect(),
            design: design.to_string(),
            cache: RefCell::new(cache),
        }
    }
    fn compute_index(&self, index: usize) -> usize {
        if self.cache.borrow().contains_key(&index) {
            return self.cache.borrow()[&index];
        }
        let mut count = 0;
        for pattern in &self.patterns {
            if index < pattern.len() - 1 {
                continue;
            }
            if &self.design[index - (pattern.len() - 1)..=index] == pattern {
                count += if index >= pattern.len() {
                    self.compute_index(index - pattern.len())
                } else {
                    1
                };
            }
        }
        self.cache.borrow_mut().insert(index, count);
        count
    }
    fn compute(&mut self) -> usize {
        self.compute_index(self.design.len() - 1)
    }
}

fn part2(patterns: &Vec<&str>, designs: &Vec<&str>) -> usize {
    designs
        .iter()
        .map(|d| PatternsUntilIndex::new(patterns, d).compute())
        .sum()
}

fn main() {
    let input_str = std::fs::read_to_string("../inputs.txt").unwrap();
    let (patterns, designs) = parse_inputs(&input_str);

    println!("Part 1: {}", part1(&patterns, &designs));
    println!("Part 2: {}", part2(&patterns, &designs));
}
