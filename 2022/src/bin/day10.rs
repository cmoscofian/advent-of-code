use aoc::get_input;

fn main() {
    let input = get_input("10");

    let first = first(&input);
    println!("first: {first}");
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops = s.split(' ').collect::<Vec<&str>>();
        match ops[0] {
            "addx" => Ok(Self::AddX(ops[1].parse::<i32>().unwrap())),
            "noop" => Ok(Self::Noop),
            _ => unreachable!(),
        }
    }
}

struct CPU {
    x: i32,
    cycle: i32,
    check: i32,
    signal: i32,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            x: 1,
            cycle: 1,
            check: 20,
            signal: 0,
        }
    }
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {
                self.evaluate(0);
            }
            Instruction::AddX(x) => {
                self.evaluate(0);
                self.evaluate(x);
            }
        }
    }

    fn evaluate(&mut self, x: i32) {
        self.cycle += 1;
        self.x += x;

        if self.cycle == self.check {
            self.signal += self.cycle * self.x;
            self.check += 40;
        }
    }
}

fn first(input: &str) -> i32 {
    let mut cpu = CPU::default();

    for line in input.lines() {
        let instruction = line.parse::<Instruction>().unwrap();
        cpu.execute(instruction);
    }

    cpu.signal
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/10/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(13140, response);
    }
}
