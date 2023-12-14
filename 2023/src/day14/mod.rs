// day14

use crate::util::{
    io,
    matrix::{self, Matrix},
};

const EMPTY: char = '.';
const ROCK: char = 'O';
const BLOCK: char = '#';

type Area = Matrix<char>;

pub fn day14() {
    println!("hello day14");

    let lines = io::read_lines("./src/day14/14-example.data").unwrap();

    let mut area: Area = Area::new(lines[0].len(), lines.len(), EMPTY);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            area.set((x, y), &c);
        }
    }

    move_rocks_up(&mut area);
    let result_a: usize = calc_value(&area);
    println!("Result A: {result_a}");

    let total_rotations = 1000000000;

    for i in 0..total_rotations {
        area = area.rotate_right();
        if i % 100000 == 0 {
            println!("{i} rotations {}%", (i * 100) / (1000000000));
        }
    }
    let result_b: usize = calc_value(&area);
    println!("Result B: {result_b}");
}

fn calc_value(area: &Matrix<char>) -> usize {
    let mut result = 0;
    for y in 0..area.ylen() {
        for x in 0..area.xlen() {
            match area.get((x, y)) {
                &ROCK => result += area.ylen() - y,
                _ => {}
            }
        }
    }
    result
}

fn move_rocks_up(area: &mut Area) {
    for y in 1..area.ylen() {
        for x in 0..area.xlen() {
            match area.get((x, y)) {
                &ROCK => {
                    let mut ypos = y;
                    let mut found = false;
                    while area.get((x, ypos - 1)) == &EMPTY {
                        found = true;
                        ypos -= 1;
                        if ypos == 0 {
                            break;
                        }
                    }
                    if found {
                        area.set((x, y), &EMPTY);
                        area.set((x, ypos), &ROCK);
                    }
                }
                _ => {}
            }
        }
    }
}
