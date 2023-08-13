pub fn part1() -> String {
    let file_path = "input.txt";
    let string = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let mut res = 0;

    format!("Answer for part1: {}", res)
}