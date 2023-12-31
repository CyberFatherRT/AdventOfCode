use std::fs;

pub fn part1() -> String {
    let file_path = "../../inputs/day4.txt";
    let string = fs::read_to_string(file_path).unwrap();
    let mut md5 = md5::compute(string.clone());
    let mut res = 1;

    while !format!("{:?}", md5).starts_with("00000") {
        res += 1;
        md5 = md5::compute(format!("{}{}", string, res).as_bytes());
    }

    format!("Answer for part1 {}", res)
}