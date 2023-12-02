use regex::Regex;
use std::fs;

fn part_1(input: &str) -> u32 {
    let regex_game = Regex::new(r"Game (\d+): (.*)").unwrap();

    input
        .lines()
        .map(|line: &str| {
            let re_match = regex_game.captures(line).unwrap();
            let game = re_match.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let rounds: Vec<&str> = re_match.get(2).unwrap().as_str().split(";").collect();
            let mut rounds_iter = rounds.into_iter();
            let mut is_invalid: bool = false;

            while let Some(round) = rounds_iter.next() {
                let items: Vec<&str> = round.trim().split_whitespace().collect();
                let mut items_iter = items.into_iter();

                while let Some(count) = items_iter.next() {
                    let total = count.parse::<u32>().unwrap();
                    let color = items_iter.next().unwrap().trim_end_matches(",");

                    if color == "red" && total > 12 {
                        is_invalid = true;
                    }
                    if color == "green" && total > 13 {
                        is_invalid = true;
                    }
                    if color == "blue" && total > 14 {
                        is_invalid = true;
                    }
                }
            }

            if is_invalid {
                0
            } else {
                game
            }
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let regex_game = Regex::new(r"Game (\d+): (.*)").unwrap();

    input
        .lines()
        .map(|line: &str| {
            let mut color_r: u32 = 0;
            let mut color_g: u32 = 0;
            let mut color_b: u32 = 0;

            let re_match = regex_game.captures(line).unwrap();
            let game = re_match.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let rounds: Vec<&str> = re_match.get(2).unwrap().as_str().split(";").collect();
            let mut rounds_iter = rounds.into_iter();

            while let Some(round) = rounds_iter.next() {
                let items: Vec<&str> = round.trim().split_whitespace().collect();
                let mut items_iter = items.into_iter();

                while let Some(count) = items_iter.next() {
                    let total = count.parse::<u32>().unwrap();
                    let color = items_iter.next().unwrap().trim_end_matches(",");

                    if color == "red" {
                        color_r = color_r.max(total);
                    }
                    if color == "green" {
                        color_g = color_g.max(total);
                    }
                    if color == "blue" {
                        color_b = color_b.max(total);
                    }
                }
            }

            color_r * color_g * color_b
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("../input/day02.txt").expect("Missing");

    println!("Day 02, Part 1 = {}", part_1(&input));
    println!("Day 02, Part 2 = {}", part_2(&input));
}
