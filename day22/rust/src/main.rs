fn evolve(number: i64) -> i64 {
    let number = ((number * 64) ^ number) % 16777216;
    let number = ((number / 32) ^ number) % 16777216;
    let number = ((number * 2048) ^ number) % 16777216;
    number
}

fn part1(numbers: &Vec<i64>) -> i64 {
    numbers
        .iter()
        .map(|n| {
            let mut n = *n;
            for _ in 0..2000 {
                n = evolve(n);
            }
            return n;
        })
        .sum()
}

fn main() {
    let numbers: Vec<i64> = std::fs::read_to_string("../inputs.txt")
        .map(|file| file.lines().map(|line| line.parse().unwrap()).collect())
        .unwrap();
    println!("Part 1: {}", part1(&numbers));
}
