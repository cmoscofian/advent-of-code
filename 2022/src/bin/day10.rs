use aoc::get_input;

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

struct Cpu {
    x: i32,
    cycle: i32,
    check: i32,
    signal: i32,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x: 1,
            cycle: 1,
            check: 20,
            signal: 0,
        }
    }
}

impl Cpu {
    fn execute(&mut self, x: i32, cycles: i32) {
        for _ in 0..cycles {
            if self.cycle == self.check {
                self.signal += self.cycle * self.x;
                self.check += 40;
            }
            self.cycle += 1;
        }

        self.x += x;
    }
}

struct Crt {
    x: i32,
    cycle: i32,
    buffer: String,
}

impl Default for Crt {
    fn default() -> Self {
        Self {
            x: 1,
            cycle: 1,
            buffer: String::new(),
        }
    }
}

impl Crt {
    fn draw(&mut self, x: i32, cycles: i32) {
        for _ in 0..cycles {
            let col = self.cycle % 40;
            let pixel = if (col - self.x - 1).abs() < 2 {
                '#'
            } else {
                '.'
            };

            self.buffer.push(pixel);
            self.cycle += 1;
            if col == 0 {
                self.buffer.push('\n');
            }
        }

        self.x += x;
    }
}

fn main() {
    let input = get_input("10");

    let first = first(&input);
    println!("first: {first}");

    let second = second(input);
    println!("second: {second}");
}

fn first(input: &str) -> i32 {
    let mut cpu = Cpu::default();

    for line in input.lines() {
        match line.parse::<Instruction>().unwrap() {
            Instruction::Noop => cpu.execute(0, 1),
            Instruction::AddX(x) => cpu.execute(x, 2),
        }
    }

    cpu.signal
}

fn second(input: String) -> String {
    let mut crt = Crt::default();

    for line in input.lines() {
        match line.parse::<Instruction>().unwrap() {
            Instruction::Noop => crt.draw(0, 1),
            Instruction::AddX(x) => crt.draw(x, 2),
        }
    }

    crt.buffer
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
