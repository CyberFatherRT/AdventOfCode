use std::fs;

pub fn part2() -> String {
    let file_path = "../../inputs/day4.txt";
    let string = fs::read_to_string(file_path).unwrap();
    let res = 0;

    format!("Answer for part2 {}", res)
}