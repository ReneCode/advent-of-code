use std::collections::HashMap;

use crate::util::io;
use itertools::Itertools;

type Number = i64;

pub fn day11() {
    let lines = io::read_lines("./src/day11/11.data").unwrap();

    let stones = lines[0]
        .split_whitespace()
        .map(|l| l.parse::<Number>().unwrap())
        .collect_vec();

    part1(&stones);
    part2(&stones);
}

fn part1(stones: &[Number]) {
    // brute force
    let mut stones = stones.to_vec();
    for _ in 0..25 {
        stones = next_stones(&stones);
    }
    println!("Day11 part 1: {:?}", stones.len());
}

pub fn next_stones(stones: &[Number]) -> Vec<Number> {
    let mut new_stones = Vec::new();

    for stone in stones {
        if *stone == 0 {
            new_stones.push(1)
        } else {
            let s = format!("{}", stone);
            let len = s.len();
            if len % 2 == 0 {
                let left = &s[0..len / 2];
                let right = &s[len / 2..len];
                new_stones.push(left.parse::<Number>().unwrap());
                new_stones.push(right.parse::<Number>().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }

    new_stones
}

fn part2(stone_array: &[Number]) {
    // create a hashmap with the stones and their count
    let mut stones = HashMap::new();
    for nr in stone_array {
        stones
            .entry(*nr)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    for _ in 0..75 {
        stones = one_blink(stones);
    }

    let total: i64 = stones.values().sum();
    println!("Day11 part 2: {:?}", total);
}

fn one_blink(stones: HashMap<Number, Number>) -> HashMap<Number, Number> {
    let mut new_stones = HashMap::new();

    // update the counts on one blink
    for (stone, org_count) in stones {
        let result = next_stone(stone);
        for nr in result {
            new_stones
                .entry(nr)
                .and_modify(|count| *count += org_count)
                .or_insert(org_count);
        }
    }
    new_stones
}

fn next_stone(stone: Number) -> Vec<Number> {
    if stone == 0 {
        vec![1]
    } else {
        let s = format!("{}", stone);
        let len = s.len();
        if len % 2 == 0 {
            let left = &s[0..len / 2];
            let right = &s[len / 2..len];
            vec![
                left.parse::<Number>().unwrap(),
                right.parse::<Number>().unwrap(),
            ]
        } else {
            vec![stone * 2024]
        }
    }
}
