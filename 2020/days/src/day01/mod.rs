use crate::util;

use itertools::Itertools;

pub fn day01() {
    println!("hello day 01");

    let lines = util::io::read_lines("./01.data").unwrap();
    // println!("data {:?}", lines);
    let numbers: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    // println!("numbers {:?}", numbers);

    if let Some(result) = solve_a(&numbers) {
        println!("result A: {result}")
    }

    if let Some(result) = solve_b(&numbers) {
        println!("result B: {result}")
    }
}

fn solve_a(numbers: &Vec<i32>) -> Option<i32> {
    let comb = numbers.iter().combinations(2);

    for c in comb {
        if c[0] + c[1] == 2020 {
            return Some(c[0] * c[1]);
        }
    }
    None
}

fn solve_b(numbers: &Vec<i32>) -> Option<i32> {
    let comb = numbers.iter().combinations(3);

    for c in comb {
        if c[0] + c[1] + c[2] == 2020 {
            return Some(c[0] * c[1] * c[2]);
        }
    }
    None
}
