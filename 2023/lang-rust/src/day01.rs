use std::fs;

pub fn part_1(input: &str) -> u32 {
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

pub fn solve() {
    let input = fs::read_to_string("../input/day01.txt").expect("Missing");

    println!("Day 01, Part 1 = {}", part_1(&input));
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::super::{*};

    #[test]
    fn test_part_1_example() {
        let input = fs::read_to_string("../input/day01-p1t.txt").expect("Missing");
        assert_eq!(day01::part_1(&input), 142);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("../input/day01.txt").expect("Missing");
        assert_eq!(day01::part_1(&input), 55108);
    }
}
