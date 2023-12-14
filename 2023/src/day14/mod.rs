// day14

use crate::util::{io, matrix::Matrix};

const EMPTY: char = '.';
const ROCK: char = 'O';
// const BLOCK: char = '#';

type Area = Matrix<char>;

struct PeriodeFinder {
    strings: Vec<String>,
}

impl PeriodeFinder {
    fn new() -> Self {
        PeriodeFinder {
            strings: Vec::new(),
        }
    }

    fn add_string(&mut self, add_str: &str) -> Option<usize> {
        self.strings.push(add_str.to_string());

        let count = self.strings.iter().filter(|s| *s == add_str).count();
        if count > 10 {
            let mut indicies: Vec<usize> = Vec::new();
            for (idx, val) in self.strings.iter().enumerate() {
                if add_str == val {
                    indicies.push(idx);
                }
            }
            // println!("indicies: {:?}", indicies);
            return Some(indicies[indicies.len() - 1] - indicies[indicies.len() - 2]);
        }
        None
    }
}

pub fn day14() {
    println!("hello day14");

    let lines = io::read_lines("./src/day14/14.data").unwrap();

    let mut orginal_area: Area = Area::new(lines[0].len(), lines.len(), EMPTY);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            orginal_area.set((x, y), &c);
        }
    }

    let mut area = orginal_area.clone();
    move_rocks_up(&mut area);
    let result_a: usize = calc_value(&area);
    println!("Result A: {result_a}");

    // part 2

    let mut finder = PeriodeFinder::new();
    let mut area = orginal_area.clone();

    let total_cycles = 1000000000;
    let mut periode_found = false;
    let mut cycle = 0;
    let mut rest_cycle = 1;
    while rest_cycle > 0 {
        cycle += 1;
        // make a complete cycle
        move_rocks_up(&mut area);
        area = area.rotate_right();
        move_rocks_up(&mut area);
        area = area.rotate_right();
        move_rocks_up(&mut area);
        area = area.rotate_right();
        move_rocks_up(&mut area);
        area = area.rotate_right();

        if !periode_found {
            if let Some(periode_len) = finder.add_string(&area.to_string().replace('\n', "|")) {
                periode_found = true;
                println!("Periode found {} on cycle {}", periode_len, cycle);
                rest_cycle = (total_cycles - cycle) % periode_len;
            }
        } else {
            rest_cycle -= 1;
        }
    }
    // println!("{}", area.to_string());

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
