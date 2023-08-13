use std::fs;

pub fn part2() -> String {
    let file_path: &str = "../../inputs/day1.txt";
    let string = fs::read_to_string(file_path).expect("Couldn't read file `input.txt`");
    let mut res = 0;
    let mut ans: String = String::new();
    for (i, elem) in string.chars().enumerate() {
        if res == -1 {
            ans = format!("Answer for part2: {}", i)
        }
        if elem == '(' {
            res += 1;
        } else if elem == ')' {
            res -= 1;
        }
    }
    ans
}
