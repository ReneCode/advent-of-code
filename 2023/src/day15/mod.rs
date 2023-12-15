// day15

use crate::util::{io, parse};

pub fn day15() {
    println!("hello day15");

    let lines = io::read_lines("./src/day15/15.data").unwrap();
    let line = lines.get(0).unwrap();
    let strings = parse::to_str(line, ',');
    let result_a: u32 = strings.iter().map(|s| calc_hash(s)).sum();

    // let result_a = calc_hash("HASH");
    println!("Result A {}", result_a);
}

fn calc_hash(s: &str) -> u32 {
    let mut hash = 0;

    for c in s.chars() {
        let ascii = c as u32;
        hash = hash + ascii;
        hash = hash * 17;
        hash = hash % 256;
    }
    hash
}
