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
            let mut f = None;
            let mut l = None;

            while x.as_str() != "" {
                let b = x.as_str();
                let a = x.next().unwrap();

                if a.is_ascii_digit() {
                    f = f.or(a.to_digit(10));
                    l = a.to_digit(10);
                } else {
                    for (idx, str) in MAPPING.iter().enumerate() {
                        if b.starts_with(str) {
                            l = Some(idx as u32 + 1);
                            f = f.or(l);
                        }
                    }
                }
            }

            f.unwrap() * 10 + l.unwrap()
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
    fn test_part_1() {
        let input = fs::read_to_string("../input/day01.txt").expect("Missing");
        assert_eq!(day01::part_2(&input), 56324);
    }
}
