use std::cmp::Ordering;
use std::ops::Index;

struct Report {
    levels: Vec<u32>,
}
impl Report {
    fn check_adj(&self, idx_a: usize, idx_b: usize, diff_sign: Option<bool>) -> bool {
        let abs_diff = self[idx_a].abs_diff(self[idx_b]);
        let sign = self[idx_a] < self[idx_b];
        0 < abs_diff && abs_diff <= 3 && diff_sign.map_or(true, |ds| ds == sign)
    }
    fn len(&self) -> usize {
        self.levels.len()
    }
}
impl Index<usize> for Report {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.levels[index]
    }
}

fn part1(reports: &[Report]) -> u32 {
    reports
        .iter()
        .map(|report| {
            let diff_sign = report[0] < report[1];
            for right_idx in 1..report.len() {
                if !report.check_adj(right_idx - 1, right_idx, Some(diff_sign)) {
                    return 0;
                }
            }
            1
        })
        .sum()
}

fn part2(reports: &[Report]) -> u32 {
    reports
        .iter()
        .map(|report| {
            match report.len().cmp(&3) {
                Ordering::Less => return 1,
                Ordering::Equal => {
                    return (report.check_adj(0, 1, None)
                        || report.check_adj(1, 2, None)
                        || report.check_adj(0, 2, None)) as u32;
                }
                _ => {}
            }
            let diff_sign: bool = [
                report.levels[0] < report.levels[1],
                report.levels[1] < report.levels[2],
                report.levels[2] < report.levels[3],
            ]
            .iter()
            .filter(|b| **b)
            .count()
                >= 2;
            let mut removed_idx: Option<usize> = None;

            if !report.check_adj(0, 1, Some(diff_sign)) {
                if report.check_adj(0, 2, Some(diff_sign)) {
                    removed_idx = Some(1);
                } else if report.check_adj(1, 2, Some(diff_sign)) {
                    removed_idx = Some(0);
                } else {
                    return 0;
                }
            }
            for right_idx in 1..report.len() - 1 {
                if removed_idx.map_or(false, |idx| idx == right_idx) {
                    continue;
                }
                let left_idx: Option<usize> =
                    if removed_idx.map_or(false, |idx| idx == right_idx - 1) {
                        if right_idx >= 2 {
                            Some(right_idx - 2)
                        } else {
                            None
                        }
                    } else {
                        Some(right_idx - 1)
                    };

                if let Some(left_idx) = left_idx {
                    if report.check_adj(left_idx, right_idx, Some(diff_sign)) {
                        continue;
                    }
                    if removed_idx.is_some() {
                        return 0;
                    }
                    if report.check_adj(left_idx - 1, right_idx, Some(diff_sign))
                        && report.check_adj(right_idx, right_idx + 1, Some(diff_sign))
                    {
                        removed_idx = Some(left_idx);
                    } else if report.check_adj(left_idx, right_idx + 1, Some(diff_sign)) {
                        removed_idx = Some(right_idx);
                    } else {
                        return 0;
                    }
                }
            }
            let penultimate = if removed_idx.map_or(false, |idx| idx == report.len() - 2) {
                report.len() - 3
            } else {
                report.len() - 2
            };
            (removed_idx.is_none()
                || report.check_adj(penultimate, report.len() - 1, Some(diff_sign)))
                as u32
        })
        .sum()
}

fn main() {
    let reports: Vec<Report> = std::fs::read_to_string("../reports.txt")
        .unwrap()
        .lines()
        .map(|line| Report {
            levels: line
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect(),
        })
        .collect();
    println!("Part 1: {}", part1(&reports));
    println!("Part 2: {}", part2(&reports));
}
