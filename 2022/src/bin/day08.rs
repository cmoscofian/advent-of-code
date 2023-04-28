use aoc::get_input;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn withing_boundaries(&self, x: i32, y: i32) -> bool {
        self.0 >= 0 && self.0 < x && self.1 >= 0 && self.1 < y
    }

    fn tree(&self, field: &[Vec<char>]) -> char {
        field[self.1 as usize][self.0 as usize]
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
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

    let day8_second = second(input);
    println!("second: {day8_second}");
}

fn first(input: &str) -> u32 {
    let field = input
        .lines()
        .map(|f| f.chars().collect())
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
            let mut tallest = start.tree(&field);

            let mut visitor = start;
            result.insert(visitor);

            while tallest < '9' && visitor.withing_boundaries(width, height) {
                let current = visitor.tree(&field);
                if current > tallest {
                    result.insert(visitor);
                    tallest = current;
                }
                visitor += look;
            }
            start += step;
        }
    }
    result.len() as u32
}

fn second(input: String) -> u32 {
    let field = input
        .lines()
        .map(|f| f.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let height = field.len() as i32;
    let width = field[0].len() as i32;

    let up = Point::from(Direction::Up);
    let down = Point::from(Direction::Down);
    let left = Point::from(Direction::Left);
    let right = Point::from(Direction::Right);

    let mut best = 0;
    for y in 0..height {
        for x in 0..width {
            let origin = Point(x, y);
            let house = origin.tree(&field);

            let mut scenic = 1;

            for step in [right, down, left, up] {
                let mut visitor = origin + step;
                let mut visible = 0;

                while visitor.withing_boundaries(width, height) {
                    visible += 1;

                    if visitor.tree(&field) >= house {
                        break;
                    }

                    visitor += step;
                }

                scenic *= visible;
            }

            if best < scenic {
                best = scenic
            }
        }
    }

    best
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

    #[test]
    fn test_second() {
        let input = read_to_string("input/08/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(8, response);
    }
}
