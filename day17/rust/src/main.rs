use regex::Regex;

#[derive(Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    i_ptr: usize,
}
impl Computer {
    fn from_string(input_str: String) -> Self {
        Computer {
            a: Regex::new(r"Register A: (\d+)")
                .unwrap()
                .captures(&input_str)
                .map(|c| c[1].parse().unwrap())
                .unwrap(),
            b: Regex::new(r"Register B: (\d+)")
                .unwrap()
                .captures(&input_str)
                .map(|c| c[1].parse().unwrap())
                .unwrap(),
            c: Regex::new(r"Register C: (\d+)")
                .unwrap()
                .captures(&input_str)
                .map(|c| c[1].parse().unwrap())
                .unwrap(),
            program: Regex::new(r"Program: (.*)$")
                .unwrap()
                .captures(&input_str)
                .map(|c| c[1].parse::<String>().unwrap())
                .unwrap()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect(),
            i_ptr: 0,
        }
    }

    fn fetch_operand(&self) -> u64 {
        self.program[self.i_ptr + 1]
    }

    fn fetch_combo_operand(&self) -> u64 {
        let op = self.program[self.i_ptr + 1];
        match op {
            0..=3 => op,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand {}", op),
        }
    }

    fn current_instruction(&self) -> Option<u64> {
        if !self.end_reached() {
            Some(self.program[self.i_ptr])
        } else {
            None
        }
    }

    fn end_reached(&self) -> bool {
        self.i_ptr >= self.program.len()
    }

    fn next_instruction(&mut self) {
        self.i_ptr += 2;
    }

    fn jump_instruction(&mut self, pos: usize) {
        self.i_ptr = pos;
    }

    fn reset(&mut self) {
        self.i_ptr = 0;
        self.a = 0;
        self.b = 0;
        self.c = 0;
    }

    fn compute_next_output(&mut self) -> Option<u64> {
        loop {
            let opcode = match self.current_instruction() {
                Some(opcode) => opcode,
                None => {
                    return None;
                }
            };

            match opcode {
                0 => {
                    let denom = self.fetch_combo_operand();
                    self.a /= 2u64.pow(denom as u32);
                    self.next_instruction();
                }
                1 => {
                    self.b = self.b ^ self.fetch_operand();
                    self.next_instruction();
                }
                2 => {
                    self.b = self.fetch_combo_operand() % 8;
                    self.next_instruction();
                }
                3 => {
                    if self.a == 0 {
                        self.next_instruction();
                        continue;
                    }
                    self.jump_instruction(self.fetch_operand() as usize);
                }
                4 => {
                    self.b ^= self.c;
                    self.next_instruction();
                }
                5 => {
                    let output = self.fetch_combo_operand() % 8;
                    self.next_instruction();
                    return Some(output);
                }
                6 => {
                    let denom = self.fetch_combo_operand();
                    self.b = self.a / 2u64.pow(denom as u32);
                    self.next_instruction();
                }
                7 => {
                    let denom = self.fetch_combo_operand();
                    self.c = self.a / 2u64.pow(denom as u32);
                    self.next_instruction();
                }
                _ => panic!("Invalid instruction {}", opcode),
            }
        }
    }

    fn compute(&mut self) -> Vec<u64> {
        let mut outputs: Vec<u64> = Vec::new();
        while let Some(output) = self.compute_next_output() {
            outputs.push(output);
        }
        outputs
    }
}

fn part1(comp: &Computer) -> String {
    let mut comp = comp.clone();
    comp.compute()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(comp: &Computer) -> u64 {
    let mut comp = comp.clone();

    let mut search_candidates: Vec<u64> = Vec::new();
    for i in 0..=0b1111111111 {
        // all 'last' 10 bits predicting the first item correctly
        comp.reset();
        comp.a = i;
        if comp.compute_next_output().unwrap() == *comp.program.first().unwrap() {
            search_candidates.push(i);
        }
    }

    let program = comp.program.clone();
    for (idx, &target) in program.iter().enumerate().skip(1) {
        let mut new_search_candidates: Vec<u64> = Vec::new();
        search_candidates.iter().for_each(|cand| {
            for new_first3 in 0..=0b111 {
                let partial_cand = ((new_first3 as u64) << 7) | cand >> (3 * idx);
                comp.reset();
                comp.a = partial_cand;
                if comp.compute_next_output().unwrap() == target {
                    new_search_candidates
                        .push(partial_cand << (3 * idx) | cand & ((1 << (3 * idx)) - 1));
                }
            }
        });
        search_candidates = new_search_candidates;
        if search_candidates.len() == 0 {
            println!("No candidates found for {}", target);
            break;
        }
    }
    *search_candidates.iter().min().unwrap()
}

fn main() {
    let input_str = std::fs::read_to_string("../inputs.txt").unwrap();
    let comp = Computer::from_string(input_str);

    println!("Part 1: {}", part1(&comp));
    println!("Part 2: {}", part2(&comp));
}
