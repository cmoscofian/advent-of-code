use std::collections::HashSet;

use aoc::get_input;

fn main() {
    let input = get_input("03");

    let day3_first = first(&input);
    println!("first: {day3_first}");

    let day3_second = second(input);
    println!("second: {day3_second}");
}

fn first(input: &str) -> u32 {
    input
        .lines()
        .map(|r| {
            let (first, second) = r.split_at(r.len() / 2);
            let cmp_1 = HashSet::<char>::from_iter(first.chars());
            let cmp_2 = HashSet::<char>::from_iter(second.chars());
            let duplicated_item = cmp_1
                .intersection(&cmp_2)
                .next()
                .expect("should return a single value from intersection");
            calculate_value(duplicated_item)
        })
        .sum::<u32>()
}

fn second(input: String) -> u32 {
    let mut result = 0;

    let mut idx = 0;
    let lines_vec = input.lines().collect::<Vec<&str>>();
    for _ in 0..input.lines().count() / 3 {
        let cmp_1 = HashSet::<char>::from_iter(
            lines_vec
                .get(idx)
                .expect("should return a valid line")
                .chars(),
        );
        let cmp_2 = HashSet::<char>::from_iter(
            lines_vec
                .get(idx + 1)
                .expect("should return a valid line")
                .chars(),
        );
        let cmp_3 = HashSet::<char>::from_iter(
            lines_vec
                .get(idx + 2)
                .expect("should return a valid line")
                .chars(),
        );

        idx += 3;

        let badge = cmp_1
            .iter()
            .filter(|k| cmp_2.contains(k))
            .find(|k| cmp_3.contains(k))
            .expect("should return a single repeated value from the rucksacks");

        result += calculate_value(badge);
    }

    result
}

fn calculate_value(c: &char) -> u32 {
    match (*c).is_uppercase() {
        true => (*c) as u32 - 38,
        false => (*c) as u32 - 96,
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

    #[test]
    fn test_second() {
        let input = read_to_string("input/03/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(70, response);
    }
}
