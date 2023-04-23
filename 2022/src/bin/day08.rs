use aoc::get_input;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn withing_boundaries(&self, x: i32, y: i32) -> bool {
        self.0 >= 0 && self.0 < x && self.1 >= 0 && self.1 < y
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point(0, -1),
            Direction::Down => Point(0, 1),
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
        }
    }
}

fn main() {
    let input = get_input("08");

    let day8_first = first(&input);
    println!("first: {day8_first}");
}

fn first(input: &str) -> u32 {
    let field = input
        .lines()
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let height = field.len() as i32;
    let width = field[0].len() as i32;

    let upper_left = Point(0, 0);
    let bottom_right = Point(width - 1, height - 1);

    let up = Point::from(Direction::Up);
    let down = Point::from(Direction::Down);
    let left = Point::from(Direction::Left);
    let right = Point::from(Direction::Right);

    let mut result = std::collections::HashSet::<Point>::new();

    for (mut start, step, look) in [
        (upper_left, right, down),
        (upper_left, down, right),
        (bottom_right, left, up),
        (bottom_right, up, left),
    ] {
        while start.withing_boundaries(width, height) {
            let mut origin = start;
            let mut tallest = field[origin.1 as usize][origin.0 as usize];

            result.insert(origin);

            while tallest < '9' && origin.withing_boundaries(width, height) {
                let current = field[origin.1 as usize][origin.0 as usize];
                if current > tallest {
                    result.insert(origin);
                    tallest = current;
                }
                origin += look;
            }
            start += step;
        }
    }
    result.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/08/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(21, response);
    }
}
