use aoc::get_input;

fn main() {
    let input = get_input("01");

    let first = first(&input);
    println!("first: {first}");
}

fn first(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut number = l.chars().filter(|c| c.is_numeric());
            let first = number.next().expect("should return a valid number");
            let last = match number.last() {
                Some(l) => l,
                None => first,
            };
            format!("{first}{last}")
                .parse::<u32>()
                .expect("should return a valid number")
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/01/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(142, response);
    }
}
