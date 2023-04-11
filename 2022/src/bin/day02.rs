use aoc::get_input;

struct Game {
    player: Hand,
    opponent: Hand,
    expected: Result,
}

impl Game {
    fn decrypt_by_hand(&self) -> u32 {
        let result = self.player.check(self.opponent);
        return self.player as u32 + result as u32;
    }

    fn decrypt_by_result(&self) -> u32 {
        let hand = self.expected.choose_hand(self.opponent);
        return self.expected as u32 + hand as u32;
    }
}

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let d = s.split(" ").collect::<Vec<_>>();

        Ok(Self {
            player: Hand::from(*d.get(1).unwrap()),
            opponent: Hand::from(*d.get(0).unwrap()),
            expected: Result::from(*d.get(1).unwrap()),
        })
    }
}

#[derive(Clone, Copy)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Hand {
    fn check(&self, opponent: Self) -> Result {
        match (self, opponent) {
            (Hand::Rock, Hand::Paper) => Result::Lose,
            (Hand::Rock, Hand::Scissors) => Result::Win,
            (Hand::Paper, Hand::Rock) => Result::Win,
            (Hand::Paper, Hand::Scissors) => Result::Lose,
            (Hand::Scissors, Hand::Rock) => Result::Lose,
            (Hand::Scissors, Hand::Paper) => Result::Win,
            (_, _) => Result::Draw,
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Result {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl Result {
    fn choose_hand(&self, opponent: Hand) -> Hand {
        match (self, opponent) {
            (Result::Win, Hand::Rock) => Hand::Paper,
            (Result::Win, Hand::Paper) => Hand::Scissors,
            (Result::Win, Hand::Scissors) => Hand::Rock,
            (Result::Lose, Hand::Rock) => Hand::Scissors,
            (Result::Lose, Hand::Paper) => Hand::Rock,
            (Result::Lose, Hand::Scissors) => Hand::Paper,
            (_, x) => x,
        }
    }
}

impl From<&str> for Result {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = get_input("02");

    let day2_first = first(&input);
    println!("day2-first: {:?}", day2_first);

    let day2_second = second(input);
    println!("day2-second: {:?}", day2_second);
}

fn first(input: &String) -> u32 {
    input
        .lines()
        .map(|x| {
            x.parse::<Game>()
                .expect("should parse into game")
                .decrypt_by_hand()
        })
        .sum()
}

fn second(input: String) -> u32 {
    input
        .lines()
        .map(|x| {
            x.parse::<Game>()
                .expect("should parse into game")
                .decrypt_by_result()
        })
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

    #[test]
    fn test_second() {
        let input = read_to_string("input/02/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(12, response);
    }
}
