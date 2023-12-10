// day09

use core::num;
use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{io, math, parse};

type Nr = i64;
type VecNr = Vec<Nr>;

pub fn day09() {
    println!("hello day09");

    let lines = io::read_lines("./src/day09/09.data").unwrap();

    let mut histories: Vec<VecNr> = Vec::new();
    for line in lines {
        let history: Vec<Nr> = parse::to_numbers(&line, ' ');
        histories.push(history);
    }

    let result_a: Nr = histories.iter().map(|h| get_value_a(h)).sum();
    println!("Result A: {result_a}");

    let result_b: Nr = histories.iter().map(|h| get_value_b(h)).sum();
    println!("Result B: {result_b}");
}

fn get_value_a(numbers: &VecNr) -> Nr {
    let mut next_numbers = get_next_numbers(numbers);
    let first_right_number = numbers[numbers.len() - 1];
    let mut right_numbers: VecNr = Vec::new();
    right_numbers.push(first_right_number);
    while !next_numbers.iter().all(|n| *n == 0) {
        // println!("next-numbers {:?}", next_numbers);

        let right_number = next_numbers[next_numbers.len() - 1];
        right_numbers.push(right_number);
        next_numbers = get_next_numbers(&next_numbers);
    }
    let sum = right_numbers.iter().sum();
    // println!("{sum}");
    sum
}

fn get_value_b(numbers: &VecNr) -> Nr {
    let mut next_numbers = get_next_numbers(numbers);
    let first_left_number = numbers[0];
    let mut left_numbers: VecNr = Vec::new();
    left_numbers.push(first_left_number);
    while !next_numbers.iter().all(|n| *n == 0) {
        let left_number = next_numbers[0];
        left_numbers.push(left_number);
        next_numbers = get_next_numbers(&next_numbers);
    }

    let mut sum = 0;
    for nr in left_numbers.iter().rev() {
        sum = nr - sum;
    }
    println!("{:?} => {sum}", left_numbers);
    sum
}

fn get_next_numbers(numbers: &VecNr) -> VecNr {
    let mut result: VecNr = Vec::new();
    for i in 0..numbers.len() - 1 {
        let diff = numbers[i + 1] - numbers[i];
        result.push(diff);
    }
    result
}
