use std::fs;

pub fn part1() -> String {
    let file_path = "../../inputs/day5.txt";
    let string = fs::read_to_string(file_path).unwrap();

    let mut res = 0;


    format!("Answer for part1: {}", res)
}