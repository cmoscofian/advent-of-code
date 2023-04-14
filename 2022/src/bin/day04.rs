use aoc::get_input;

struct Section {
    first: (u32, u32),
    second: (u32, u32),
}

impl Section {
    fn check_contains(self) -> bool {
        self.first.0 >= self.second.0 && self.first.1 <= self.second.1
            || self.first.0 <= self.second.0 && self.first.1 >= self.second.1
    }

    fn check_overlaps(self) -> bool {
        self.first.0 >= self.second.0 && self.first.0 <= self.second.1
            || self.second.0 >= self.first.0 && self.second.0 <= self.first.1
    }
}

impl std::str::FromStr for Section {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(',')
            .flat_map(|f| f.split('-').map(|x| x.parse::<u32>().unwrap()))
            .collect::<Vec<_>>();

        let a = values.get(0).unwrap().clone();
        let b = values.get(1).unwrap().clone();
        let c = values.get(2).unwrap().clone();
        let d = values.get(3).unwrap().clone();

        Ok(Self {
            first: if a < b { (a, b) } else { (b, a) },
            second: if c < d { (c, d) } else { (d, c) },
        })
    }
}

fn main() {
    let input = get_input("04");

    let day4_first = first(&input);
    println!("first: {day4_first}");

    let day4_second = second(input);
    println!("second: {day4_second}");
}

fn first(input: &String) -> u32 {
    input
        .lines()
        .filter(|x| x.parse::<Section>().unwrap().check_contains())
        .count() as u32
}

fn second(input: String) -> u32 {
    input
        .lines()
        .filter(|x| x.parse::<Section>().unwrap().check_overlaps())
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/04/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(2, response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/04/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(4, response);
    }
}
