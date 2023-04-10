use aoc::get_input;

fn main() {
    let input = get_input("01");
    let day1_first = first(&input);
    println!("day1-first: {:?}", day1_first);
}

fn first(input: &String) -> u32 {
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
}
