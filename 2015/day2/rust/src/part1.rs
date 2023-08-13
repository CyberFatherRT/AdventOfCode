pub fn part1() -> String {
    let file_path = "input.txt";
    let string = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let mut res = 0;

    for i in string.trim_end().split('\n') {
        let i = i.split("x").map(|x| x.parse().expect("Failed to parse")).collect::<Vec<i32>>();
        let (l, w, h) = (i[0], i[1], i[2]);
        let area1 = l * w;
        let area2 = w * h;
        let area3 = h * l;
        res += 2 * (area1 + area2 + area3) + vec![area1, area2, area3].iter().min().unwrap();
    }
    format!("Answer for part1: {}", res)
}