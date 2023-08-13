use std::collections::HashSet;

pub fn part1() -> String {
    let file_path = "../../inputs/day3.txt";
    let string = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let mut res = 1;
    let (mut x, mut y) = (0, 0);
    let mut set: HashSet<(i32, i32)> = HashSet::from([(x, y)]);

    for elem in string.chars() {
        match elem {
            '^' => { y += 1 },
            'v' => { y -= 1 },
            '>' => { x += 1 },
            '<' => { x -= 1 },
            _ => unreachable!()
        }
        if set.insert((x, y)) {
            res += 1;
        }
    }

    format!("Answer for part1: {}", res)
}