use std::fs;

fn main() {
    let file_path: &str = "input.txt";
    let string = fs::read_to_string(file_path).expect("Couldn't read file `input.txt`");
    let mut ans = 0;
    for i in string.chars() {
        if i == '(' {
            ans += 1;
        } else if i == ')' {
            ans -= 1;
        }
    }
    println!("{}", ans);
}
