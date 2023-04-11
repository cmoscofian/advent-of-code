use std::collections::HashSet;

use aoc::get_input;

fn main() {
    let input = get_input("03");

    let day3_first = first(&input);
    println!("first: {day3_first}");
}

fn first(input: &String) -> u32 {
    input
        .lines()
        .map(|r| {
            let (first, second) = r.split_at(r.len() / 2);
            let cmp_1 = std::collections::HashSet::<char>::from_iter(first.chars());
            let cmp_2 = std::collections::HashSet::<char>::from_iter(second.chars());
            let inter = cmp_1.intersection(&cmp_2).collect::<HashSet<&char>>();
            let repeated_item = inter
                .iter()
                .next()
                .expect("should return a single value from intersection");
            calculate_value(repeated_item)
        })
        .sum::<u32>()
}

fn calculate_value(c: &&char) -> u32 {
    match (**c).is_uppercase() {
        true => (**c) as u32 - 38,
        false => (**c) as u32 - 96,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/03/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(157, response);
    }
}
