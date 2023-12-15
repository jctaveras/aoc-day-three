use core::str::Lines;
use fancy_regex::{Captures, Error, Regex};
use std::{fs, iter::Enumerate};

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Input file should exist.");
    let symbol_regex = Regex::new(r"[^\.\d]").unwrap();
    let gear_regex = Regex::new(r"\*").unwrap();
    let results = vec![symbol_regex, gear_regex]
        .iter()
        .map(|r| {
            content
                .lines()
                .enumerate()
                .map(|line_value| get_point(line_value, r.clone()))
                .flatten()
                .map(|reference_point| {
                    find_part_number(reference_point, content.lines().enumerate())
                })
                .collect()
        })
        .collect::<Vec<Vec<Vec<u32>>>>();

    println!(
        "The sum of all part numbers is: {}",
        results[0].iter().flatten().sum::<u32>()
    );
    println!(
        "The product of all of the gear ratios is: {}",
        results[1]
            .iter()
            .map(|gear_ratios| {
                if gear_ratios.len() > 1 {
                    gear_ratios.iter().product::<u32>()
                } else {
                    0
                }
            })
            .sum::<u32>()
    );
}

fn get_point(value: (usize, &str), symbol_regex: Regex) -> Vec<Coordinate> {
    let line = value.1;
    let y_value = value.0;

    symbol_regex
        .captures_iter(line)
        .map(|result| create_point(result, y_value))
        .collect()
}

fn create_point(result: Result<Captures<'_>, Error>, y: usize) -> Coordinate {
    match result {
        Ok(capture_char) => match capture_char.get(0) {
            Some(found_char) => Coordinate {
                x: found_char.start(),
                y: y,
            },
            None => panic!("No character found."),
        },
        Err(e) => panic!("{e}"),
    }
}

fn find_part_number(reference_point: Coordinate, lines: Enumerate<Lines>) -> Vec<u32> {
    let part_number_regex = Regex::new(r"(\d+)").unwrap();

    lines
        .filter(|line| {
            line.0 == reference_point.y
                || (line.0 == (reference_point.y - 1))
                || (line.0 == (reference_point.y + 1))
        })
        .map(|line| {
            part_number_regex
                .captures_iter(line.1)
                .map(|result| get_part_number(result, reference_point.x))
                .filter(|number| *number > 0)
                .collect::<Vec<u32>>()
        })
        .flatten()
        .collect()
}

fn get_part_number(result: Result<Captures, Error>, x: usize) -> u32 {
    match result {
        Ok(part) => match part.get(1) {
            Some(part_number) => {
                if part_number.start() == x
                    || part_number.end() == x
                    || part_number.start() == (x + 1)
                    || part_number.start() == (x - 1)
                    || part_number.end() == (x + 1)
                {
                    parse_value(part_number.as_str().trim())
                } else {
                    0
                }
            }
            None => 0,
        },
        Err(e) => panic!("{e}"),
    }
}

fn parse_value(value: &str) -> u32 {
    match value.parse() {
        Ok(number) => number,
        Err(_) => panic!("Part should be a number"),
    }
}
