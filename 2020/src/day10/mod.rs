// day10

use std::{collections::HashMap, vec};

use crate::util::io;

pub fn day10() {
    let lines = io::read_lines("10-example.data").unwrap();

    let numbers: Vec<i64> = lines
        .iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let part_1 = calc_steps(&numbers);
    println!("Part 1: {}", part_1);

    let part_2 = part2(&numbers);
    println!("Part 2: {}", part_2);
}

fn calc_steps(numbers: &Vec<i64>) -> i64 {
    let mut sorted = numbers.clone();
    sorted.sort();

    let mut diff_1 = 0;
    let mut diff_3 = 0;
    let mut last_nr = 0;
    for nr in sorted {
        let diff = nr - last_nr;
        match diff {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => println!("Error"),
        }
        last_nr = nr;
    }

    // output is 3 higher
    diff_3 += 1;
    // println!("Diff 1: {}, Diff 3: {}", diff_1, diff_3);
    diff_1 * diff_3
}

fn part2(numbers: &Vec<i64>) -> i64 {
    let mut sorted = numbers.clone();
    sorted.sort();

    let mut count: u64 = 0;

    let mut steps: HashMap<i64, i64> = HashMap::new();
    for nr in sorted.iter().rev() {
        let possibilities = get_possibilities(&sorted, *nr);
        if possibilities.len() == 0 {
            steps.insert(*nr, 1);
        } else {
            let mut sum = 0;
            for p in possibilities {
                sum += steps.get(&p).unwrap();
            }
            steps.insert(*nr, sum);
        }
    }

    for nr in sorted.iter() {
        println!("{}: {}", nr, steps.get(nr).unwrap());
    }
    // println!("{:?}", steps);

    let first_nr = sorted[0];
    let result = steps.get(&first_nr).unwrap();

    *result
}

fn get_possibilities(numbers: &Vec<i64>, from: i64) -> Vec<i64> {
    let mut possibilities: Vec<i64> = Vec::new();
    for i in 1..4 {
        if numbers.contains(&(from + i)) {
            possibilities.push(from + i);
        }
    }

    possibilities
}
