use regex::RegexBuilder;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Arcade {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

fn math_optimum_arcade_prize(arcade: &Arcade) -> Option<u64> {
    let x11 = arcade.button_a.x as f64;
    let x21 = arcade.button_a.y as f64;
    let x12 = arcade.button_b.x as f64;
    let x22 = arcade.button_b.y as f64;

    let prize_x = arcade.prize.x as f64;
    let prize_y = arcade.prize.y as f64;

    let det = x11 * x22 - x12 * x21;
    let a = (x22 * prize_x - x12 * prize_y) / det;
    let b = (x11 * prize_y - x21 * prize_x) / det;

    if a.fract() == 0.0 && b.fract() == 0.0 && a >= 0.0 && b >= 0.0 {
        return Some(3 * a as u64 + b as u64);
    }
    None
}

fn part1(arcades: &[Arcade]) -> u64 {
    arcades.iter().filter_map(math_optimum_arcade_prize).sum()
}
fn part2(arcades: &[Arcade]) -> u64 {
    let arcades: Vec<Arcade> = arcades
        .iter()
        .map(|a| Arcade {
            button_a: a.button_a,
            button_b: a.button_b,
            prize: Point {
                x: a.prize.x + 10000000000000,
                y: a.prize.y + 10000000000000,
            },
        })
        .collect();
    part1(&arcades)
}

fn main() {
    let arcade_regex = RegexBuilder::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .multi_line(true)
    .build()
    .unwrap();
    let input_str = std::fs::read_to_string("../arcade.txt").unwrap();
    let arcades: Vec<Arcade> = arcade_regex
        .captures_iter(&input_str)
        .map(|c| Arcade {
            button_a: Point {
                x: c[1].parse().unwrap(),
                y: c[2].parse().unwrap(),
            },
            button_b: Point {
                x: c[3].parse().unwrap(),
                y: c[4].parse().unwrap(),
            },
            prize: Point {
                x: c[5].parse().unwrap(),
                y: c[6].parse().unwrap(),
            },
        })
        .collect();
    println!("{:?}", part1(&arcades));
    println!("{:?}", part2(&arcades));
}
