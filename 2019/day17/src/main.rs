use std::collections::HashSet;

use util::{io, point::Point};

mod int_computer;

const OUT_SCAFFOLD: char = '#';
const OUT_FREE: char = '.';
const OUT_NEW_LINE: char = '\n';
const OUT_DIRECTION_UP: char = '^';
const OUT_DIRECTION_DOWN: char = 'v';
const OUT_DIRECTION_LEFT: char = '<';
const OUT_DIRECTION_RIGHT: char = '>';

struct Collector {
    field: Vec<Vec<char>>,
    // scaffold: HashSet<Point>,
    // free: HashSet<Point>,
    current_location: Point,
}

impl Collector {
    fn new() -> Self {
        Collector {
            field: Vec::new(),
            current_location: Point::new(0, 0),
        }
    }

    fn add_point(&mut self, value: char) {
        let pt = &self.current_location;
        if self.field.len() <= pt.y as usize {
            self.field.push(Vec::new())
        }
        self.field[pt.y as usize].push(value);
    }

    fn detect_intersections(&self) -> i32 {
        // intersection is, if from a point pt the up,down,left,right points also exists

        let y_len = self.field.len();
        let x_len = self.field[0].len();
        let mut intersections: Vec<Point> = Vec::new();
        for y in 1..y_len - 1 {
            for x in 1..x_len - 1 {
                let val = self.field[y][x];
                if val == OUT_SCAFFOLD {
                    if self.field[y - 1][x] == OUT_SCAFFOLD
                        && self.field[y + 1][x] == OUT_SCAFFOLD
                        && self.field[y][x - 1] == OUT_SCAFFOLD
                        && self.field[y][x + 1] == OUT_SCAFFOLD
                    {
                        intersections.push(Point::new(x as i32, y as i32));
                    }
                }
            }
        }

        let mut result = 0;
        for pt in intersections.iter() {
            result += pt.x * pt.y
        }
        result
    }

    fn print(&self) {
        for row in &self.field {
            let mut line = String::new();
            for val in row {
                line.push(*val)
            }
            println!("{line}")
        }
    }
}

impl int_computer::TakeInputOutput for Collector {
    fn read_input(&mut self) -> i64 {
        todo!()
    }

    fn take_output(&mut self, value: i64) {
        let c = char::from(value as u8);
        match c {
            OUT_FREE => self.add_point(c),
            OUT_SCAFFOLD => self.add_point(c),
            OUT_NEW_LINE => {
                self.current_location.y += 1;
                self.current_location.x = -1;
            }
            OUT_DIRECTION_UP => self.add_point(c),
            OUT_DIRECTION_DOWN => self.add_point(c),
            OUT_DIRECTION_LEFT => self.add_point(c),
            OUT_DIRECTION_RIGHT => self.add_point(c),
            _ => panic!("bad value {c}"),
        }
        self.current_location.x += 1;
    }
}

fn main() {
    println!("Hello, day17!");
    if let Some(lines) = io::get_lines("./17.data") {
        let line = &lines[0];
        part_1(&line);
    }
}

fn part_1(line: &str) {
    let mut computer = int_computer::IntComputer::new(line);
    let mut collector = Collector::new();
    computer.run(&mut collector);
    collector.print();
    let result = collector.detect_intersections();
    println!("part1 sum of the alignment parameters {}", result);
}
