use aoc::get_input;

#[derive(Default)]
struct Rope {
    knots: Vec<(i32, i32)>,
    visited: std::collections::HashSet<(i32, i32)>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); knots],
            visited: std::collections::HashSet::new(),
        }
    }

    fn walk(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.knots[0].1 += 1,
            Direction::Down => self.knots[0].1 -= 1,
            Direction::Right => self.knots[0].0 += 1,
            Direction::Left => self.knots[0].0 -= 1,
        }

        for i in 1..self.knots.len() {
            let delta_x = self.knots[i - 1].0 - self.knots[i].0;
            let delta_y = self.knots[i - 1].1 - self.knots[i].1;

            if delta_x.abs() > 1 || delta_y.abs() > 1 {
                self.knots[i].0 += delta_x.signum();
                self.knots[i].1 += delta_y.signum();
            }

            self.visited.insert(self.knots[self.knots.len() - 1]);
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => unreachable!("invalid direction chars"),
        }
    }
}

fn main() {
    let input = get_input("09");

    let first = first(&input);
    println!("first: {first}");

    let second = second(input);
    println!("second: {second}");
}

fn first(input: &str) -> usize {
    let mut rope = Rope::new(2);

    for line in input.lines() {
        let data = line.split(' ').collect::<Vec<&str>>();
        for _ in 0..data[1].parse().unwrap() {
            rope.walk(data[0].parse().unwrap());
        }
    }

    rope.visited.len()
}

fn second(input: String) -> usize {
    let mut rope = Rope::new(10);

    for line in input.lines() {
        let data = line.split(' ').collect::<Vec<&str>>();
        for _ in 0..data[1].parse().unwrap() {
            rope.walk(data[0].parse().unwrap());
        }
    }

    rope.visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/09/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(13, response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/09/example2").expect("should read file successfully");
        let response = second(input);
        assert_eq!(36, response);
    }
}
