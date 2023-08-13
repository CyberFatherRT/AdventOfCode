use std::fs;

pub fn part1() -> String {
    let file_path = "../../inputs/day5.txt";
    let string = fs::read_to_string(file_path).unwrap();

    let mut ans = 0;

    for line in string.lines() {
        let mut res = 0;
        let mut prev: char = ' ';
        let (mut flag1, mut flag2, mut flag3) = (true, true, false);
        let mut vowels = 0;

        for c in line.chars() {
            if "aeiou".contains(c) {
                vowels += 1;
            }
            if c == prev && flag2 {
                res += 1;
                flag2 = false;
            }
            if ["ab", "cd", "pq", "xy"].contains(&&*format!("{}{}", prev, c)) {
                flag3 = true;
                break;
            }

            if vowels == 3 && flag1 {
                res += 1;
                flag1 = false;
            }

            prev = c;
        }
        if flag3 {
            continue;
        }
        if res == 2 {
            ans += 1;
        }
    }

    format!("Answer for part1: {}", ans)
}
