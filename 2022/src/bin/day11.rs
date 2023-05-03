use aoc::get_input;

#[derive(Default)]
enum Operation {
    #[default]
    AddSelf,
    Add(i64),
    MultiplySelf,
    Multiply(i64),
}

#[derive(Default)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: i64,
    destination: (usize, usize),
    inspections: usize,
}

impl Monkey {
    fn operate(&self, value: i64) -> i64 {
        match self.operation {
            Operation::AddSelf => value + value,
            Operation::Add(x) => value + x,
            Operation::MultiplySelf => value * value,
            Operation::Multiply(x) => value * x,
        }
    }
}

impl std::str::FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut monkey = Monkey::default();

        for line in s.lines() {
            let words = line.trim().split(' ').collect::<Vec<&str>>();
            match words[0] {
                "Starting" => {
                    let (_, list) = line.trim().split_once(": ").unwrap();
                    monkey.items = list
                        .split(',')
                        .map(|f| f.trim().parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                }
                "Operation:" => {
                    monkey.operation = match (words[4], words[5]) {
                        ("+", "old") => Operation::AddSelf,
                        ("+", num) => Operation::Add(num.parse::<i64>().unwrap()),
                        ("*", "old") => Operation::MultiplySelf,
                        ("*", num) => Operation::Multiply(num.parse::<i64>().unwrap()),
                        _ => unreachable!(),
                    }
                }
                "Test:" => {
                    monkey.test = words[3].parse::<i64>().unwrap();
                }
                "If" => match words[1] {
                    "true:" => {
                        monkey.destination.0 = words[5].parse::<usize>().unwrap();
                    }
                    "false:" => {
                        monkey.destination.1 = words[5].parse::<usize>().unwrap();
                    }
                    _ => unreachable!(),
                },
                _ => {}
            }
        }

        Ok(monkey)
    }
}

fn main() {
    let input = get_input("11");

    let first = first(&input);
    println!("first: {first}");

    let second = second(input);
    println!("second: {second}");
}

fn first(input: &str) -> usize {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let mut monkeys = data
        .iter()
        .map(|f| f.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                let nitem = monkeys[i].operate(item) / 3;
                let dest = if nitem % monkeys[i].test == 0 {
                    monkeys[i].destination.0
                } else {
                    monkeys[i].destination.1
                };
                monkeys[i].inspections += 1;
                monkeys[dest].items.push(nitem);
            }
        }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();

    monkey_business.sort_by(|a, b| b.cmp(a));
    monkey_business[0] * monkey_business[1]
}

fn second(input: String) -> usize {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let mut monkeys = data
        .iter()
        .map(|f| f.parse::<Monkey>().unwrap())
        .collect::<Vec<Monkey>>();

    let modval = monkeys.iter().map(|m| m.test).product::<i64>();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while let Some(item) = monkeys[i].items.pop() {
                // Shamelessly stolen from someone else...
                // I don't fully get this, why would modding the least common multiple would yield
                // the same solution?
                let nitem = monkeys[i].operate(item) % modval;

                let dest = if nitem % monkeys[i].test == 0 {
                    monkeys[i].destination.0
                } else {
                    monkeys[i].destination.1
                };
                monkeys[i].inspections += 1;
                monkeys[dest].items.push(nitem);
            }
        }
    }

    let mut monkey_business = monkeys
        .iter()
        .map(|m| m.inspections)
        .collect::<Vec<usize>>();

    monkey_business.sort_by(|a, b| b.cmp(a));
    monkey_business[0] * monkey_business[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/11/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(10605, response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/11/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(2713310158, response);
    }
}
