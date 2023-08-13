use std::fs;

pub fn part2() -> String {
    let file_path: &str = "input.txt";
    let string = fs::read_to_string(file_path).expect("Couldn't read file `input.txt`");
    let mut ans = 0;
    for (i, elem) in string.chars().enumerate() {
        if ans == -1 {
            return format!("Answer for part2: {}", i)
        }
        if elem == '(' {
            ans += 1;
        } else if elem == ')' {
            ans -= 1;
        }
    }
    String::from("No answer for part2")
}