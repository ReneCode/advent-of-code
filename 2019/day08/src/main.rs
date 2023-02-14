extern crate util;
use itertools::Itertools;

const PIXEL_BLACK: i32 = 0;
const PIXEL_WHITE: i32 = 1;
const PIXEL_TRANSPARENT: i32 = 2;

type Layer = Vec<i32>;

fn main() {
    println!("Hello, day08!");

    if let Some(input) = util::io::get_lines("./08.data") {
        let line = &input[0];
        part_1(line, 25, 6);
        let result = part_2(line, 25, 6);
        output_result_part2(&result, 25);
    }
}

#[test]
fn test_1() {
    let line = "0222112222120000";
    assert_eq!(vec![0, 1, 1, 0], part_2(line, 2, 2));
}

fn get_layers(line: &str, width: usize, height: usize) -> Vec<Layer> {
    let picture_size: usize = width * height;
    let numbers = line.chars().map(|c| c as i32 - '0' as i32).collect_vec();
    let layers = numbers
        .chunks(picture_size)
        .map(|slice| slice.to_vec())
        .collect_vec();
    layers
}

fn part_1(line: &str, width: usize, height: usize) {
    let mut layers = get_layers(line, width, height);
    layers.sort_by(|a, b| {
        // sort from less to many
        let a_count_zeros: usize = count_numbers(a, 0);
        let b_count_zeros: usize = count_numbers(b, 0);
        a_count_zeros.cmp(&b_count_zeros)
    });
    let min_layer = &layers[0];
    let cnt_1 = count_numbers(&min_layer, 1);
    let cnt_2 = count_numbers(&min_layer, 2);
    println!("part-1 result: {}", cnt_1 * cnt_2);
}

fn count_numbers(layer: &Layer, find: i32) -> usize {
    layer.iter().filter(|n| **n == find).count()
}

fn part_2(line: &str, width: usize, height: usize) -> Layer {
    let layers = get_layers(line, width, height);

    let mut result_layer: Layer = Vec::new();
    let picture_size: usize = width * height;
    for idx in 0..picture_size {
        let result_pixel = get_result_pixel(&layers, idx);
        result_layer.push(result_pixel);
    }
    result_layer
}

fn output_result_part2(result: &Layer, width: usize) {
    println!("part-2 result:");
    let rows = result.chunks(width);
    for row in rows {
        let line = row
            .iter()
            .map(|val| match *val {
                PIXEL_BLACK => ' ',
                PIXEL_WHITE => '#',
                _ => {
                    panic!("bad pixel")
                }
            })
            .fold("".to_string(), |mut acc, ch| {
                acc.push(ch);
                acc
            });
        println!("{}", line);
    }
}

fn get_result_pixel(layers: &Vec<Layer>, idx: usize) -> i32 {
    let mut result = PIXEL_TRANSPARENT;
    for layer in layers {
        match layer.get(idx) {
            Some(&PIXEL_BLACK) => {
                result = PIXEL_BLACK;
                break;
            }
            Some(&PIXEL_WHITE) => {
                result = PIXEL_WHITE;
                break;
            }
            _ => {}
        }
    }
    result
}
