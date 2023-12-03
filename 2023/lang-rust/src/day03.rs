use regex::Regex;
use std::fs;

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

fn get_adjacent_coords(
    symbol_coords: (usize, usize),
    number_coords: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut coords: Vec<(usize, usize)> = Vec::new();
    let (symbol_x, symbol_y) = symbol_coords;

    for (x, y) in NEIGHBORS.iter() {
        let new_x = (symbol_x as isize + x) as usize;
        let new_y = (symbol_y as isize + y) as usize;

        if number_coords.contains(&(new_x, new_y)) {
            coords.push((new_x, new_y));
        }
    }

    coords
}

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

fn part_2(input: &str) -> u32 {
    let mut characters_coords_hit: Vec<(usize, usize)> = Vec::new();
    let mut characters: Vec<Vec<char>> = Vec::new();
    let mut symbol_coords: Vec<(usize, usize)> = Vec::new();
    let mut number_coords: Vec<(usize, usize)> = Vec::new();
    let mut result: u32 = 0;

    for (y, line) in input.lines().enumerate() {
        let mut line_characters: Vec<char> = Vec::new();

        for (x, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                number_coords.push((x, y));
            }

            if !char.is_digit(10) && char != '.' {
                symbol_coords.push((x, y));
            }

            line_characters.push(char);
        }

        characters.push(line_characters);
    }

    for coords in symbol_coords.iter() {
        let adjacent_coords = get_adjacent_coords(coords.clone(), number_coords.clone());
        let mut adjacent_values: Vec<u32> = Vec::new();

        for (x, y) in adjacent_coords.iter() {
            let vec_len = characters[*y].len() - 1;
            let mut val = String::new();
            let mut ptr = *x;

            if !characters_coords_hit.contains(&(ptr, *y)) {
                val.push(characters[*y][ptr]);
            }

            characters_coords_hit.push((ptr, *y));

            // move backwards
            while ptr > 0
                && characters[*y][ptr - 1].is_digit(10)
                && !characters_coords_hit.contains(&(ptr - 1, *y))
            {
                characters_coords_hit.push((ptr - 1, *y));
                val.insert(0, characters[*y][ptr - 1]);
                ptr -= 1;
            }

            // reset ptr
            ptr = *x;

            // move forwards
            while ptr < vec_len
                && characters[*y][ptr + 1].is_digit(10)
                && !characters_coords_hit.contains(&(ptr + 1, *y))
            {
                characters_coords_hit.push((ptr + 1, *y));
                val.push(characters[*y][ptr + 1]);
                ptr += 1;
            }

            if val.len() > 0 {
                adjacent_values.push(val.parse().unwrap());
            }
        }

        if adjacent_values.len() > 1 {
            let symbol_product: u32 = adjacent_values.iter().product();

            result += symbol_product;
        }

        adjacent_values.clear();
    }

    result
}

pub fn solve() {
    let input = fs::read_to_string("../input/day03.txt").expect("Missing");

    println!("Day 03, Part 1 = {}", part_1(&input));
    println!("Day 03, Part 2 = {}", part_2(&input));
}
