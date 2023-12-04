use std::fs;

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let mut data = line.split(": ").nth(1).unwrap().split(" | ");
            let mut numbers = data.next().unwrap().split_ascii_whitespace();
            let winning: Vec<&str> = data.next().unwrap().split_ascii_whitespace().collect();
            let mut points = 0;

            while let Some(number) = numbers.next() {
                if winning.contains(&number) {
                    points = match points {
                        0 => 1,
                        _ => points * 2,
                    }
                }
            }

            points
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let mut cards = vec![1; input.lines().count()];

    for (n, line) in input.lines().enumerate() {
        let mut data = line.split(": ").nth(1).unwrap().split(" | ");
        let mut numbers = data.next().unwrap().split_ascii_whitespace();
        let winning: Vec<&str> = data.next().unwrap().split_ascii_whitespace().collect();

        let mut cards_won = 0;

        while let Some(number) = numbers.next() {
            if winning.contains(&number) {
                cards_won += 1;
            }
        }

        for x in (n + 1)..=(cards_won + n) {
            cards[x] += cards[n];
        }
    }

    cards.iter().sum()
}

pub fn solve() {
    let input = fs::read_to_string("../input/day04.txt").expect("Missing");

    println!("Day 04, Part 1 = {}", part_1(&input));
    println!("Day 04, Part 2 = {}", part_2(&input));
}
