pub fn get_input(day: &str) -> String {
    let path = format!("input/{day}/data");
    std::fs::read_to_string(path).expect("should read file successfully")
}
