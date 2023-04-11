use aoc::get_input;

struct Game {
    player: Hand,
    opponent: Hand,
}

enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Result {
    Victory = 6,
    Draw = 3,
    Lost = 0,
}

impl Game {
    fn play(&self) -> u32 {
        match (&self.player, &self.opponent) {
            (Hand::Rock, Hand::Rock) => Hand::Rock as u32 + Result::Draw as u32,
            (Hand::Rock, Hand::Paper) => Hand::Rock as u32 + Result::Lost as u32,
            (Hand::Rock, Hand::Scissors) => Hand::Rock as u32 + Result::Victory as u32,
            (Hand::Paper, Hand::Rock) => Hand::Paper as u32 + Result::Victory as u32,
            (Hand::Paper, Hand::Paper) => Hand::Paper as u32 + Result::Draw as u32,
            (Hand::Paper, Hand::Scissors) => Hand::Paper as u32 + Result::Lost as u32,
            (Hand::Scissors, Hand::Rock) => Hand::Scissors as u32 + Result::Lost as u32,
            (Hand::Scissors, Hand::Paper) => Hand::Scissors as u32 + Result::Victory as u32,
            (Hand::Scissors, Hand::Scissors) => Hand::Scissors as u32 + Result::Draw as u32,
        }
    }
}

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let d = s.split(" ").collect::<Vec<_>>();

        Ok(Self {
            player: Hand::from(*d.get(1).unwrap()),
            opponent: Hand::from(*d.get(0).unwrap()),
        })
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Rock,
            "X" => Self::Rock,
            "B" => Self::Paper,
            "Y" => Self::Paper,
            "C" => Self::Scissors,
            "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = get_input("02");

    let day2_first = first(&input);
    println!("day2-first: {:?}", day2_first);
}

fn first(input: &String) -> u32 {
    input
        .lines()
        .map(|x| x.parse::<Game>().expect("should parse into game").play())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/02/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(15, response);
    }
}
