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

fn second(input: String) -> u32 {
    let nums = [
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let mut result = 0;

    for line in input.lines() {
        let mut first = None;
        'first: for i in 0..line.len() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if line[i..i + num.len()] == **num {
                    first = Some(1 + index / 2);
                    break 'first;
                }
            }
        }

        let mut last = None;
        'last: for i in (0..line.len()).rev() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if line[i..i + num.len()] == **num {
                    last = Some(1 + index / 2);
                    break 'last;
                }
            }
        }

        result += 10 * first.unwrap() as u32 + last.unwrap() as u32;
    }
    result
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

    #[test]
    fn test_second() {
        let input = read_to_string("input/01/example2").expect("should read file successfully");
        let response = second(input);
        assert_eq!(281, response);
    }
}
