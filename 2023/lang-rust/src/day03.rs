use regex::Regex;
use std::fs;

fn part_1(input: &str) -> u32 {
    let re_digits = Regex::new(r"(\d+)").unwrap();
    let re_symbol = Regex::new(r"(\!|\@|\#|\$|\%|\^|\&|\*|\_|\-|\\|\+|\/|\=)").unwrap();

    let mut digits_prev: Vec<regex::Match> = vec![];
    let mut digits_curr: Vec<regex::Match> = vec![];
    let mut symbol_prev: Vec<usize> = vec![];
    let mut symbol_curr: Vec<usize> = vec![];

    input
        .lines()
        .map(|line: &str| {
            let mut digits = re_digits.captures_iter(line);
            let mut symbol = re_symbol.captures_iter(line);
            let mut output: u32 = 0;

            digits_prev = digits_curr.clone();
            digits_curr = vec![];
            symbol_prev = symbol_curr.clone();
            symbol_curr = vec![];

            while let Some(item) = digits.next() {
                let value = item.get(0).unwrap();
                let range = value.range();

                digits_curr.push(value);

                for pos in &symbol_prev {
                    if range.contains(&(pos))
                        || range.contains(&(pos - 1))
                        || range.contains(&(pos + 1))
                    {
                        output = output + value.as_str().parse::<u32>().unwrap();
                    }
                }
            }

            while let Some(item) = symbol.next() {
                let pos = item.get(0).unwrap().start();

                symbol_curr.push(pos);

                for prev in &digits_prev {
                    let range = prev.range();

                    if range.contains(&(pos))
                        || range.contains(&(pos - 1))
                        || range.contains(&(pos + 1))
                    {
                        output = output + prev.as_str().parse::<u32>().unwrap();
                    }
                }

                for curr in &digits_curr {
                    let range = curr.range();

                    if range.contains(&(pos))
                        || range.contains(&(pos - 1))
                        || range.contains(&(pos + 1))
                    {
                        output = output + curr.as_str().parse::<u32>().unwrap();
                    }
                }
            }

            output
        })
        .sum()
}

pub fn solve() {
    let input = fs::read_to_string("../input/day03.txt").expect("Missing");

    println!("Day 03, Part 1 = {}", part_1(&input));
    // println!("Day 02, Part 2 = {}", part_2(&input));
}
