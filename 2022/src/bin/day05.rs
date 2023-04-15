use aoc::get_input;

fn main() {
    let input = get_input("05");

    let day5_first = first(&input);
    println!("first: {day5_first}");

    let day5_second = second(input);
    println!("second: {day5_second}");
}

struct Ship {
    keys: Vec<usize>,
    data: std::collections::HashMap<usize, std::collections::VecDeque<char>>,
}

impl std::str::FromStr for Ship {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = std::collections::HashMap::<usize, std::collections::VecDeque<char>>::new();
        let mut keys = Vec::new();
        s.lines().rev().skip(1).for_each(|f| {
            f.char_indices().for_each(|(i, c)| match c {
                'A'..='Z' | 'a'..='z' => {
                    if let Some(value) = data.get_mut(&i) {
                        value.push_back(c);
                    } else {
                        let mut deque = std::collections::VecDeque::new();
                        deque.push_back(c);
                        data.insert(i, deque);
                        keys.push(i);
                    };
                }
                _ => {}
            })
        });

        keys.sort();

        Ok(Self { data, keys })
    }
}

impl Ship {
    fn createmover9000(&mut self, instruction: Instruction) {
        for _ in 0..instruction.moves {
            let from_key = self.keys.get(instruction.from - 1).unwrap();
            let to_key = self.keys.get(instruction.to - 1).unwrap();

            let from_stack = self.data.get_mut(from_key).unwrap();
            let value = from_stack.pop_back().unwrap();

            let to_stack = self.data.get_mut(to_key).unwrap();
            to_stack.push_back(value);
        }
    }

    fn createmover9001(&mut self, instruction: Instruction) {
        let from_key = self.keys.get(instruction.from - 1).unwrap();
        let from_stack = self.data.get_mut(from_key).unwrap();

        let elements = from_stack
            .drain(from_stack.len() - instruction.moves..)
            .collect::<Vec<char>>();

        let to_key = self.keys.get(instruction.to - 1).unwrap();
        let to_stack = self.data.get_mut(to_key).unwrap();

        to_stack.extend(elements);
    }

    fn get_top_of_stack(&mut self) -> String {
        let mut resp = String::new();

        self.keys.iter().for_each(|f| {
            let vec = self.data.get(f).unwrap();
            resp.push(*(vec.back().unwrap()));
        });

        resp
    }
}

struct Instruction {
    moves: usize,
    from: usize,
    to: usize,
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.split(' ').collect::<Vec<&str>>();
        let moves = data.get(1).unwrap().parse::<usize>().unwrap();
        let from = data.get(3).unwrap().parse::<usize>().unwrap();
        let to = data.get(5).unwrap().parse::<usize>().unwrap();
        Ok(Self { moves, from, to })
    }
}

fn first(input: &str) -> String {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let mut ship = data.first().unwrap().parse::<Ship>().unwrap();
    let instructions = data
        .get(1)
        .unwrap()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    for instr in instructions {
        ship.createmover9000(instr);
    }

    ship.get_top_of_stack()
}

fn second(input: String) -> String {
    let data = input.split("\n\n").collect::<Vec<&str>>();
    let mut ship = data.first().unwrap().parse::<Ship>().unwrap();
    let instructions = data
        .get(1)
        .unwrap()
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect::<Vec<Instruction>>();

    for instr in instructions {
        ship.createmover9001(instr);
    }

    ship.get_top_of_stack()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/05/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!("CMZ", response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/05/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!("MCD", response);
    }
}
