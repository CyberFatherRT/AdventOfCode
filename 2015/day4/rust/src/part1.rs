use std::fs;

pub fn part1() -> String {
    let file_path = "../../inputs/day4.txt";
    let mut string = fs::read_to_string(file_path).unwrap();
    let mut md5 = md5::compute(string.clone());
    let mut res = 1;

    while !md5.0.starts_with("00000".as_ref()) {
        md5 = md5::compute(format!("{}{}", string, res).as_bytes());
        res += 1;
        println!("{}{}", string, res);
        break
    }

    format!("Answer for part1 {}", res)
}