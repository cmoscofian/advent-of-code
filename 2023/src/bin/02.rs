use aoc::get_input;

struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl Round {
    fn is_valid(&self, other: &Round) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

impl std::str::FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        s.split(", ")
            .for_each(|l| match l.split_once(' ').unwrap() {
                (n, "red") => red = n.parse::<u32>().unwrap(),
                (n, "green") => green = n.parse::<u32>().unwrap(),
                (n, "blue") => blue = n.parse::<u32>().unwrap(),
                (_, _) => unreachable!(),
            });

        Ok(Self { red, blue, green })
    }
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid(&self, boundary: &Round) -> bool {
        self.rounds.iter().filter(|r| !r.is_valid(boundary)).count() == 0
    }

    fn min_required_cubes(&self) -> Round {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for round in &self.rounds {
            red = red.max(round.red);
            blue = blue.max(round.blue);
            green = green.max(round.green);
        }

        Round { red, blue, green }
    }
}

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_first, _second) = s.split_once(": ").unwrap();
        let (_, _id) = _first.split_once(' ').unwrap();
        let id = _id.parse::<u32>().unwrap();

        let rounds = _second
            .split("; ")
            .map(|r| r.parse::<Round>().unwrap())
            .collect::<Vec<Round>>();

        Ok(Self { id, rounds })
    }
}

fn first(input: &str) -> u32 {
    let boundary = Round {
        red: 12,
        blue: 14,
        green: 13,
    };
    input
        .lines()
        .map(|l| l.parse::<Game>().unwrap())
        .filter(|g| g.is_valid(&boundary))
        .map(|g| g.id)
        .sum()
}

fn second(input: String) -> u32 {
    input
        .lines()
        .map(|l| {
            let game = l.parse::<Game>().unwrap();
            let cubes = game.min_required_cubes();
            cubes.red * cubes.green * cubes.blue
        })
        .sum()
}

fn main() {
    let input = get_input("02");

    let first = first(&input);
    println!("first: {first}");

    let second = second(input);
    println!("second: {second}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/02/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(8, response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/02/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(2286, response);
    }
}
