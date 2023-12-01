use std::fs;

const MAPPING: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part_1(input: &str) -> u32 {
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

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let mut x = line.chars();
            let mut digit_1 = None;
            let mut digit_2 = None;

            while x.as_str() != "" {
                let string = x.as_str();
                let char = x.next().unwrap();

                if char.is_ascii_digit() {
                    digit_2 = char.to_digit(10);
                    digit_1 = digit_1.or(digit_2);
                } else {
                    for (idx, str) in MAPPING.iter().enumerate() {
                        if string.starts_with(str) {
                            digit_2 = Some(idx as u32 + 1);
                            digit_1 = digit_1.or(digit_2);
                        }
                    }
                }
            }

            digit_1.unwrap() * 10 + digit_2.unwrap()
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("../input/day01.txt").expect("Missing");

    println!("Day 01, Part 1 = {}", part_1(&input));
    println!("Day 01, Part 2 = {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use std::fs;

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

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("../input/day01.txt").expect("Missing");
        assert_eq!(day01::part_2(&input), 56324);
    }
}
