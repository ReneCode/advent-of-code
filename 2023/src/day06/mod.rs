// day06

use core::num;
use std::{collections::HashSet, string};

use itertools::Itertools;

use crate::util::{io, parse};

pub fn day06() {
    println!("hello day06");

    let lines = io::read_lines("./src/day06/06.data").unwrap();

    let tok = parse::to_str(&lines[0], ':');
    let times = parse::to_numbers::<i32>(&tok[1], ' ');

    let tok = parse::to_str(&lines[1], ':');
    let distances = parse::to_numbers::<i32>(&tok[1], ' ');

    // println!("{:?} {:?}", times, distances);

    let mut counts: Vec<i32> = Vec::new();
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];

        let distances = get_distances(time);
        let ok_distances = distances.iter().filter(|d| **d > distance).collect_vec();
        // println!("{time}/{distance} => {:?}", &ok_distances);
        counts.push(ok_distances.len() as i32)
    }
    // println!("{:?}", counts);
    let result_a: i32 = counts.iter().product();
    println!("Result A: {result_a}");

    let one_time: i64 = times
        .iter()
        .map(|n| format!("{n}"))
        .join("")
        .parse()
        .unwrap();
    let one_distance: i64 = distances
        .iter()
        .map(|n| format!("{n}"))
        .join("")
        .parse()
        .unwrap();

    let result_b = (1..one_time)
        .map(|t| t * (one_time - t))
        .filter(|v| *v > one_distance)
        .count();

    println!("Result B: {result_b}");
}

fn get_distances(time: i32) -> Vec<i32> {
    (1..time).map(|t| t * (time - t)).collect_vec()
}
