// day10

use crate::util::io;

pub fn day10() {
    let lines = io::read_lines("10-example.data").unwrap();

    let numbers: Vec<i32> = lines
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let part_1 = calc_steps(&numbers);
    println!("Part 1: {}", part_1);

    let part_2 = part2(&numbers);
    println!("Part 2: {}", part_2);
}

fn calc_steps(numbers: &Vec<i32>) -> i32 {
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

fn part2(numbers: &Vec<i32>) -> i32 {
    let mut sorted = numbers.clone();
    sorted.sort();

    let mut paths = vec![0; sorted.len()];
    paths[0] = 1;

    0
}
