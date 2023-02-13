extern crate util;
use itertools::Itertools;

fn main() {
    println!("Hello, day08!");

    if let Some(input) = util::io::get_lines("./08.data") {
        let line = &input[0];
        part_1(line, 25, 6);

        part_2();
    }
}

#[test]
fn test_1() {}

fn part_1(line: &str, width: usize, height: usize) {
    let picture_size: usize = width * height;
    let numbers = line.chars().map(|c| c as i32 - '0' as i32).collect_vec();
    let mut layers = numbers.chunks(picture_size).collect_vec();

    layers.sort_by(|a, b| {
        // sort from less to many
        let a_count_zeros: usize = count_numbers(*a, 0);
        let b_count_zeros: usize = count_numbers(*b, 0);
        a_count_zeros.cmp(&b_count_zeros)
    });
    let min_layer = layers[0];
    let cnt_0 = count_numbers(min_layer, 0);
    let cnt_1 = count_numbers(min_layer, 1);
    let cnt_2 = count_numbers(min_layer, 2);
    println!("part-1 result: {}", cnt_1 * cnt_2);
}

fn count_numbers(numbers: &[i32], find: i32) -> usize {
    numbers.iter().filter(|n| **n == find).count()
}

fn part_2() {}
