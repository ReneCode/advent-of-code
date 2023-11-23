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

    if let Some(result) = solve(numbers) {
        println!("result: {result}")
    }
}

fn solve(numbers: Vec<i32>) -> Option<i32> {
    let comb = numbers.iter().combinations(2);

    for c in comb {
        if c[0] + c[1] == 2020 {
            return Some(c[0] * c[1]);
        }
    }
    None
}
