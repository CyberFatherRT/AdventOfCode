mod part1;
mod part2;

use part1::part1;
use part2::part2;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("{} --- time: {:?}", part1(), start.elapsed());

    let start = Instant::now();
    println!("{} --- time: {:?}", part2(), start.elapsed());
}
