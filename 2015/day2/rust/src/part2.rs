use itertools::Itertools;


pub fn part2() -> String {
    let file_path = "input.txt";
    let string = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let mut res = 0;

    for i in string.trim_end().split('\n') {
        let i = i.split("x").map(|x| x.parse().expect("Failed to parse")).sorted().collect::<Vec<i32>>();
        let (l, w, h) = (i[0], i[1], i[2]);
        res += l + l + w + w + l * w * h;
    }
    format!("Answer for part2: {}", res)
}