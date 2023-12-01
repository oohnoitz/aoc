use std::fs;

fn part_a(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let f = digits.next().unwrap();
            let l = digits.last().unwrap_or(f);

            f * 10 + l
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("../input/day01.txt").expect("Missing");
    let input_p1t = fs::read_to_string("../input/day01-p1t.txt").expect("Missing");

    println!("Part A - Test = {}", part_a(&input_p1t));
    println!("Part A = {}", part_a(&input));
}
