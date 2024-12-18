use regex::Regex;

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
}

fn main() {
    let input_str = std::fs::read_to_string("../inputs.txt").unwrap();
    let mut comp = Computer::from_string(input_str);

    let mut outputs: Vec<u64> = Vec::new();

    loop {
        let opcode = match comp.current_instruction() {
            Some(opcode) => opcode,
            None => break,
        };

        match opcode {
            0 => {
                let denom = comp.fetch_combo_operand();
                comp.a /= 2u64.pow(denom as u32);
                comp.next_instruction();
            }
            1 => {
                comp.b = comp.b ^ comp.fetch_operand();
                comp.next_instruction();
            }
            2 => {
                comp.b = comp.fetch_combo_operand() % 8;
                comp.next_instruction();
            }
            3 => {
                if comp.a == 0 {
                    comp.next_instruction();
                    continue;
                }
                comp.jump_instruction(comp.fetch_operand() as usize);
            }
            4 => {
                comp.b ^= comp.c;
                comp.next_instruction();
            }
            5 => {
                outputs.push(comp.fetch_combo_operand() % 8);
                comp.next_instruction();
            }
            6 => {
                let denom = comp.fetch_combo_operand();
                comp.b = comp.a / 2u64.pow(denom as u32);
                comp.next_instruction();
            }
            7 => {
                let denom = comp.fetch_combo_operand();
                comp.c = comp.a / 2u64.pow(denom as u32);
                comp.next_instruction();
            }
            _ => panic!("Invalid instruction {}", opcode),
        }
    }

    println!(
        "Part 1: {}",
        outputs
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
}
