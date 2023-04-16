use aoc::get_input;

fn main() {
    let input = get_input("06");

    let day6_first = first(&input);
    println!("first: {day6_first}");
}

fn first(input: &str) -> u32 {
    let chars = input.char_indices().collect::<Vec<(usize, char)>>();

    for ch in chars.windows(4) {
        let mut set = std::collections::HashSet::<char>::new();
        let chars = ch.iter().map(|(_, c)| *c).collect::<Vec<_>>();
        set.extend(chars);
        if set.len() == 4 {
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
}
