use aoc::get_input;

fn main() {
    let input = get_input("06");

    let first = first(&input);
    println!("first: {first}");

    let second = second(input);
    println!("second: {second}");
}

fn first(input: &str) -> u32 {
    let chars = input.char_indices().collect::<Vec<(usize, char)>>();

    for ch in chars.windows(4) {
        let mut set = std::collections::HashSet::<char>::new();
        let chars = ch.iter().map(|(_, c)| *c).collect::<Vec<char>>();
        set.extend(chars);
        if set.len() == 4 {
            return ch.last().unwrap().0 as u32 + 1;
        }
    }

    unreachable!();
}

fn second(input: String) -> u32 {
    let chars = input.char_indices().collect::<Vec<(usize, char)>>();

    for ch in chars.windows(14) {
        let mut set = std::collections::HashSet::<char>::new();
        let chars = ch.iter().map(|(_, c)| *c).collect::<Vec<char>>();
        set.extend(chars);
        if set.len() == 14 {
            return ch.last().unwrap().0 as u32 + 1;
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/06/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(7, response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/06/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(19, response);
    }
}
