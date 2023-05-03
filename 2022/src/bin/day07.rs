use aoc::get_input;

#[derive(Default)]
struct File<'a> {
    size: std::cell::RefCell<u32>,
    parent: std::rc::Weak<File<'a>>,
    children: std::cell::RefCell<std::collections::HashMap<&'a str, std::rc::Rc<File<'a>>>>,
}

impl<'a> File<'a> {
    fn calculate_size(&self) -> u32 {
        *self.size.borrow()
            + self
                .children
                .borrow()
                .values()
                .fold(0, |a, f| a + f.calculate_size())
    }
}

fn parse_filetree(input: &str) -> std::rc::Rc<File> {
    let root = std::rc::Rc::new(File::default());
    let mut parent = std::rc::Rc::clone(&root);

    for line in input.lines() {
        let args = line.split(' ').collect::<Vec<&str>>();
        match (args[0], args[1]) {
            ("$", "ls") => {}
            ("$", "cd") => {
                parent = match args[2] {
                    "/" => std::rc::Rc::clone(&root),
                    ".." => std::rc::Rc::clone(&parent.parent.upgrade().unwrap()),
                    name => parent.children.borrow().get(name).unwrap().clone(),
                }
            }
            ("dir", name) => {
                parent.children.borrow_mut().insert(
                    name,
                    std::rc::Rc::new(File {
                        size: std::cell::RefCell::new(0),
                        parent: std::rc::Rc::downgrade(&parent),
                        children: std::cell::RefCell::new(std::collections::HashMap::new()),
                    }),
                );
            }
            (size, _) => {
                *parent.size.borrow_mut() += size.parse::<u32>().unwrap();
            }
        }
    }
    root
}

fn main() {
    let input = get_input("07");

    let first = first(&input);
    println!("first: {first}");

    let second = second(input);
    println!("second: {second}");
}

fn first(input: &str) -> u32 {
    let root = parse_filetree(input);

    let mut tree = vec![std::rc::Rc::clone(&root)];
    let mut result = 0;

    while let Some(current) = tree.pop() {
        let children = current
            .children
            .borrow()
            .values()
            .map(std::rc::Rc::clone)
            .collect::<Vec<std::rc::Rc<File>>>();
        tree.extend(children);

        let value = current.calculate_size();
        if value <= 100_000 {
            result += value;
        }
    }
    result
}

fn second(input: String) -> u32 {
    const FILESYSTEM: u32 = 70_000_000;
    const REQUIRED: u32 = 30_000_000;

    let root = parse_filetree(&input);
    let system_size = root.calculate_size();
    let free_space = FILESYSTEM - system_size;

    let mut tree = vec![std::rc::Rc::clone(&root)];
    let mut result = 0;

    while let Some(current) = tree.pop() {
        let children = current
            .children
            .borrow()
            .values()
            .map(std::rc::Rc::clone)
            .collect::<Vec<std::rc::Rc<File>>>();

        tree.extend(children);

        let value = current.calculate_size();
        if value >= (REQUIRED - free_space) && (value <= result || result == 0) {
            result = value;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_first() {
        let input = read_to_string("input/07/example").expect("should read file successfully");
        let response = first(&input);
        assert_eq!(95437, response);
    }

    #[test]
    fn test_second() {
        let input = read_to_string("input/07/example").expect("should read file successfully");
        let response = second(input);
        assert_eq!(24933642, response);
    }
}
