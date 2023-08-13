use std::collections::HashSet;

pub fn part2() -> String {
    let file_path = "../../inputs/day3.txt";
    let string = std::fs::read_to_string(file_path).expect("Failed to read input file");

    let mut flag = true;

    let mut set1 = HashSet::new();
    set1.insert((1, 1));

    let mut set2 = HashSet::new();
    set2.insert((1, 1));

    let (mut x1, mut y1) = (0, 0);
    let (mut x2, mut y2) = (0, 0);

    for elem in string.chars() {
        let (x, y, set) = if flag {
            (&mut x1, &mut y1, &mut set1)
        } else {
            (&mut x2, &mut y2, &mut set2)
        };
        flag = !flag;
        match elem {
            '^' => { *y += 1 },
            'v' => { *y -= 1 },
            '>' => { *x += 1 },
            '<' => { *x -= 1 },
            _ => unreachable!()
        }
        set.insert((*x, *y));
    }

    format!("Answer for part2: {}", set1.union(&set2).count())
}