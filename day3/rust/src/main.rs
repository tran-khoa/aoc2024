use regex::Regex;

fn part1(instructions: &str) -> i64 {
    let re = Regex::new(r".*?mul\(([0-9]+),([0-9]+)\).*?").unwrap();
    re.captures_iter(instructions)
        .map(|capture| {
            let x = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
            x * y
        })
        .sum()
}

fn part2(instructions: &str) -> i64 {
    let re_valid_section = Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    re_valid_section
        .find_iter(instructions)
        .map(|section| part1(section.as_str()))
        .sum()
}

fn main() {
    let instructions = std::fs::read_to_string("../instructions.txt")
        .unwrap()
        .replace('\n', "");
    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}
