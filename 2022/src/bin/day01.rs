use aoc::get_input;

fn main() {
    let input = get_input("01");

    let first = first(&input);
    println!("first: {first}");

    let second = second(input);
    println!("second: {second}");
}

fn first(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elve| {
            elve.lines()
                .map(|x| x.parse::<u32>().expect("should parse string"))
                .sum::<u32>()
        })
        .max()
        .expect("should return the max value")
}

fn second(input: String) -> u32 {
    let mut resp = input
        .split("\n\n")
        .map(|elve| {
            elve.lines()
                .map(|x| x.parse::<u32>().expect("should parse string"))
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    resp.sort_by(|x, y| y.cmp(x));
    resp.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/01/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(24000, response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/01/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(45000, response);
    }
}
