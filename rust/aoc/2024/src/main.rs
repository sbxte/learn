pub mod days;
pub mod util;

fn main() {
    println!("AOC 2024!");
    println!(
        "Day 1 part 2: {}",
        days::day1::part2(&util::get_input("day1.txt"))
    );
    println!(
        "Day 1 part 1: {}",
        days::day1::part1(&util::get_input("day1.txt"))
    );
}
