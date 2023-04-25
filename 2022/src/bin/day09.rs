use aoc::get_input;

#[derive(Default)]
struct Rope {
    head: (i32, i32),
    tail: (i32, i32),
    visited: std::collections::HashSet<(i32, i32)>,
}

impl Rope {
    fn walk(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.head.1 += 1,
            Direction::Down => self.head.1 -= 1,
            Direction::Right => self.head.0 += 1,
            Direction::Left => self.head.0 -= 1,
        }

        let delta_x = self.head.0 - self.tail.0;
        let delta_y = self.head.1 - self.tail.1;

        let abs_x = delta_x.abs();
        let abs_y = delta_y.abs();

        if abs_x > 1 && delta_y == 0 {
            self.tail.0 += delta_x.signum();
        }

        if abs_y > 1 && delta_x == 0 {
            self.tail.1 += delta_y.signum();
        }

        if abs_x > 1 && abs_y > 0 || abs_y > 1 && abs_x > 0 {
            self.tail.0 += delta_x.signum();
            self.tail.1 += delta_y.signum();
        }

        self.visited.insert(self.tail);
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

    let day9_first = first(&input);
    println!("first: {day9_first}");
}

fn first(input: &str) -> usize {
    let mut rope = Rope::default();

    for line in input.lines() {
        let (direction, moves) = line.split_once(' ').unwrap();
        for _ in 0..moves.parse().unwrap() {
            rope.walk(direction.parse().unwrap());
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
}
