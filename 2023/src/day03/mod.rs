// day03

use core::num;
use std::collections::HashSet;

use crate::util::{
    io,
    point2d::{self, Point2d},
};

#[derive(Debug)]
struct Number {
    value: i32,
    x_start: usize,
    x_stop: usize,
    y: usize,
}

pub fn day03() {
    println!("hello day03");

    let lines = io::read_lines("./src/day03/03.data").unwrap();
    let numbers = get_numbers(&lines);
    let valid_numbers = get_valid_numbers(&lines, &numbers);
    // println!(">>> valid numbers {:?}", valid_numbers);
    let result: i32 = valid_numbers.iter().sum();
    println!("Result A: {result}");

    let gear_symbols = get_gear_symbols(&lines);
    let result_b = get_result_b(&numbers, &gear_symbols);

    println!("Result B: {result_b}")
}

fn is_number_nearby(number: &Number, pt: &Point2d) -> bool {
    for x in number.x_start..number.x_stop + 1 {
        let num_pt = Point2d::new(x, number.y);
        if num_pt.is_near(pt) {
            return true;
        }
    }
    false
}

fn get_result_b(numbers: &[Number], gear_symbols: &[Point2d]) -> i32 {
    let mut result = 0;
    for gear_symbol in gear_symbols {
        let mut numbers_nearby: Vec<i32> = Vec::new();
        for number in numbers {
            if is_number_nearby(number, gear_symbol) {
                numbers_nearby.push(number.value)
            }
        }
        if numbers_nearby.len() == 2 {
            let gear_ratio: i32 = numbers_nearby.iter().product();
            result += gear_ratio;
        }
    }
    result
}

fn get_gear_symbols(lines: &[String]) -> Vec<Point2d> {
    let mut result: Vec<Point2d> = Vec::new();
    for y in 0..lines.len() {
        let line = &lines[y];
        for x in 0..line.len() {
            let c = line.chars().nth(x).unwrap();
            if c == '*' {
                result.push(Point2d::new(x, y));

                // println!(">>>> symbol: {c} {x} {y}")
            }
        }
    }
    result
}

fn is_symbol(lines: &[String], x: usize, y: usize) -> bool {
    let line = &lines[y];
    let c = line.chars().nth(x).unwrap();
    !c.is_digit(10) && c != '.'
}

fn has_adjacent_to_symbol(lines: &[String], number: &Number) -> bool {
    let x_max = lines[0].len();
    let y_max = lines.len();
    let mut neighbours: HashSet<Point2d> = HashSet::new();
    for x in number.x_start..(number.x_stop + 1) {
        let nb = point2d::get_neighbours(&Point2d::new(x, number.y), x_max, y_max);
        neighbours.extend(nb);
    }
    // println!(">> neighbours: {:?} {:?}", number, neighbours);

    for neighbour in neighbours {
        if is_symbol(lines, neighbour.x, neighbour.y) {
            return true;
        }
    }

    false
}

fn get_valid_numbers(lines: &[String], numbers: &[Number]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for number in numbers {
        if has_adjacent_to_symbol(lines, number) {
            result.push(number.value)
        }
    }
    result
}

fn get_numbers(lines: &Vec<String>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for y in 0..lines.len() {
        let line = &lines[y];

        let mut start_idx = 0;
        while start_idx < line.len() {
            if let Some(x_start) = get_first_digit_index(&line, start_idx) {
                if let Some(x_stop) = get_last_digit_index(&line, x_start) {
                    let value: i32 = line[x_start..x_stop + 1].parse().unwrap();
                    let number = Number {
                        x_start: x_start,
                        x_stop: x_stop,
                        y: y,
                        value: value,
                    };
                    // println!(">> Number {:?}", number);
                    numbers.push(number);
                    start_idx = x_stop + 1;
                }
            } else {
                break;
            }
        }
    }

    // println!("numbers {:?}", numbers);
    numbers
}

fn get_last_digit_index(line: &str, x_start: usize) -> Option<usize> {
    for idx in x_start..line.len() {
        if !line.chars().nth(idx).unwrap().is_digit(10) {
            return Some(idx - 1);
        }
    }
    Some(line.len() - 1)
}

fn get_first_digit_index(line: &str, x_start: usize) -> Option<usize> {
    for idx in x_start..line.len() {
        if line.chars().nth(idx).unwrap().is_digit(10) {
            return Some(idx);
        }
    }
    None
}
