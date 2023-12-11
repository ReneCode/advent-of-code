// day11

use core::num;
use std::{collections::HashMap, ops::IndexMut};

use itertools::Itertools;

use crate::util::{io, math, parse};

const EMPTY: char = '.';
const GALAXY: char = '#';

pub fn day11() {
    println!("hello day11");

    let lines = io::read_lines("./src/day11/11.data").unwrap();
    let empty_rows = get_empty_row(&lines);
    let empty_cols = get_empty_cols(&lines);

    println!(
        "empty rows: {:?} / empty cols: {:?}",
        empty_rows, empty_cols
    );

    let mut galaxies = get_galaxies(&lines);
    galaxies = expand_universe(&galaxies, &empty_rows, &empty_cols, 1);
    let combinations = galaxies.iter().combinations(2);

    let result_a: usize = combinations
        .map(|comb| get_distance(comb[0], comb[1]))
        .sum();

    // println!("galaxies: {:?}", galaxies);
    println!("Result A: {result_a}");
    // let galaxy = expand_universe(&lines, empty_rows, empty_cols);

    // part B

    let mut galaxies = get_galaxies(&lines);
    galaxies = expand_universe(&galaxies, &empty_rows, &empty_cols, 1000000 - 1);
    let combinations = galaxies.iter().combinations(2);

    let result_a: usize = combinations
        .map(|comb| get_distance(comb[0], comb[1]))
        .sum();

    // println!("galaxies: {:?}", galaxies);
    println!("Result A: {result_a}");
    // let galaxy = expand_universe(&lines, empty_rows, empty_cols);
}

fn count_lq(vals: &Vec<usize>, val: usize) -> usize {
    vals.iter().filter(|v| **v <= val).count()
}

fn expand_universe(
    galaxies: &[(usize, usize)],
    empty_rows: &Vec<usize>,
    empty_cols: &Vec<usize>,
    factor: usize,
) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (x, y) in galaxies.iter() {
        let expand_x = count_lq(&empty_cols, *x);
        let expand_y = count_lq(&empty_rows, *y);
        result.push((x + factor * expand_x, y + factor * expand_y))
    }
    result
}

fn get_galaxies(lines: &[String]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == GALAXY {
                result.push((x, y))
            }
        }
    }
    result
}

fn get_distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let distance = a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
    // println!("{:?} {:?} -> {distance}", a, b);
    distance
}

fn get_empty_row(lines: &[String]) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if line.chars().all(|c| c == EMPTY) {
            result.push(i);
        }
    }
    result
}

fn get_empty_cols(lines: &[String]) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let width = lines[0].len();
    for i in 0..width {
        let mut col_empty = lines
            .iter()
            .all(|line| line.chars().nth(i).unwrap() == EMPTY);
        if col_empty {
            result.push(i);
        }
    }
    result
}
