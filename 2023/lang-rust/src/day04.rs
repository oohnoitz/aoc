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
    let mut cards: Vec<Vec<usize>> = Vec::new();
    let mut total: u32 = 0;

    for (n, line) in input.lines().enumerate() {
        let mut data = line.split(": ").nth(1).unwrap().split(" | ");
        let mut numbers = data.next().unwrap().split_ascii_whitespace();
        let winning: Vec<&str> = data.next().unwrap().split_ascii_whitespace().collect();

        let mut card: Vec<usize> = Vec::new();
        let mut t = 0;

        while let Some(number) = numbers.next() {
            if winning.contains(&number) {
                t += 1;
                card.push(n + t);
            }
        }

        cards.push(card)
    }

    for (n, _card) in input.lines().enumerate() {
        total = total_winnings(&mut (total + 1), cards.clone(), n);
    }

    total
}

fn total_winnings(total: &mut u32, cards: Vec<Vec<usize>>, n: usize) -> u32 {
    let winnings = &cards[n];

    if winnings.len() > 0 {
        for i in winnings {
            *total = total_winnings(&mut (*total + 1), cards.clone(), *i);
        }
    }

    *total
}

pub fn solve() {
    let input = fs::read_to_string("../input/day04.txt").expect("Missing");

    println!("Day 04, Part 1 = {}", part_1(&input));
    println!("Day 04, Part 2 = {}", part_2(&input));
}
