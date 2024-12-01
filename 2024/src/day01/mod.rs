use crate::util::io;

pub fn day01() {
    let lines = io::read_lines("./src/day01/01.data").unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let tok: Vec<&str> = line.split(" ").filter(|t| t.len() > 0).collect();
        let number = tok[0].trim().parse::<i32>().unwrap();
        left.push(number);
        let number = tok[1].trim().parse::<i32>().unwrap();
        right.push(number);
    }
    println!("Day 01!");
    right.sort();
    left.sort();

    let mut sum = 0;
    for idx in 0..left.len() {
        let delta = (left[idx] - right[idx]).abs();
        sum += delta;
    }

    println!("Part 1: {}", sum);

    let mut count = 0;
    for nr in left {
        let similarity = right.iter().filter(|n| **n == nr).count() as i32;
        count += (similarity * nr);
    }
    println!("Part 2: {}", count);
}
