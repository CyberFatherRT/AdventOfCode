use std::process::{Command, Stdio};

pub fn part2() -> String {
    let file_path = "../../inputs/day5.txt";

    let cat = Command::new("cat")
        .arg(file_path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep1 = Command::new("grep")
        .arg(r"\(..\).*\1")
        .stdin(Stdio::from(cat.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep2 = Command::new("grep")
        .arg(r"\(.\).\1")
        .stdin(Stdio::from(grep1.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let wc = Command::new("wc")
        .arg("-l")
        .stdin(Stdio::from(grep2.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output = wc.wait_with_output().unwrap();
    let ans = String::from_utf8(output.stdout).unwrap();

    format!("Answer for part2: {}", ans.trim())
}
