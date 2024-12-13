use regex::RegexBuilder;
use std::ops;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn l1(&self) -> usize {
        self.x + self.y
    }
}
impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl ops::Sub<Point> for Point {
    type Output = Option<Point>;
    fn sub(self, other: Point) -> Option<Point> {
        if self.x < other.x || self.y < other.y {
            return None;
        }
        Some(Point {
            x: self.x - other.x,
            y: self.y - other.y,
        })
    }
}
impl ops::Mul<usize> for Point {
    type Output = Point;
    fn mul(self, other: usize) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl ops::Div<usize> for Point {
    type Output = Option<Point>;
    fn div(self, other: usize) -> Option<Point> {
        if self.x % other != 0 || self.y % other != 0 {
            return None;
        }
        Some(Point {
            x: self.x / other,
            y: self.y / other,
        })
    }
}
impl ops::BitOr<Point> for Point {
    type Output = Option<usize>;
    fn bitor(self, other: Point) -> Option<usize> {
        /*
        If vector A is 'divided by' vector B, return factor
         */
        let factor = self.x / other.x;
        if other.x * factor == self.x && other.y * factor == self.y {
            return Some(factor);
        }
        None
    }
}

#[derive(Debug)]
struct Arcade {
    button_a: Point,
    button_b: Point,
    prize: Point,
}

fn optimum_arcade_prize(arcade: &Arcade) -> Option<usize> {
    let (expensive_vec, cheap_vec, cheap_cost, expensive_cost) =
        if arcade.button_a.l1() * 3 > arcade.button_b.l1() {
            (arcade.button_a, arcade.button_b, 1, 3)
        } else {
            (arcade.button_b, arcade.button_a, 3, 1)
        };
    let mut num_expensive: usize = 0;
    loop {
        let remainder = arcade.prize - expensive_vec * num_expensive;
        if let Some(remainder) = remainder {
            if let Some(num_cheap) = remainder | cheap_vec {
                return Some(cheap_cost * num_cheap + expensive_cost * num_expensive);
            }
        } else {
            return None;
        }
        num_expensive += 1;
    }
}

fn part1(arcades: &Vec<Arcade>) -> usize {
    arcades.iter().filter_map(|a| optimum_arcade_prize(a)).sum()
}

fn naive_integer_decomp(n: usize) -> Vec<usize> {
    let mut factors = vec![];
    let mut n = n;
    let mut d = 2;
    while d * d < n {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
    }
    factors
}

fn common_factors(n: usize, m: usize) -> Vec<usize> {
    let decomp_m = naive_integer_decomp(m);

    naive_integer_decomp(n)
        .iter()
        .filter(|&f| decomp_m.contains(f))
        .cloned()
        .collect()
}

fn part2(arcades: &Vec<Arcade>) {
    let arcades: Vec<Arcade> = arcades
        .iter()
        .map(|a| Arcade {
            button_a: a.button_a,
            button_b: a.button_b,
            prize: a.prize
                + Point {
                    x: 10000000000000,
                    y: 10000000000000,
                },
        })
        .collect();
    // wip
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
