use std::collections::VecDeque;

use util::{io, point::Point};

mod int_computer;
use int_computer::IntComputer;

const OUT_SCAFFOLD: char = '#';
const OUT_FREE: char = '.';
const OUT_NEW_LINE: char = '\n';
const OUT_DIRECTION_UP: char = '^';
const OUT_DIRECTION_DOWN: char = 'v';
const OUT_DIRECTION_LEFT: char = '<';
const OUT_DIRECTION_RIGHT: char = '>';

struct Collector {
    field: Vec<Vec<char>>,
    current_location: Point,
    input: VecDeque<i64>,
    debug: bool,
    output: i64,
}

impl Collector {
    fn new() -> Self {
        Collector {
            field: Vec::new(),
            current_location: Point::new(0, 0),
            input: VecDeque::new(),
            debug: false,
            output: 0,
        }
    }

    fn add_input(&mut self, input: &str) {
        for c in input.chars() {
            self.input.push_back(c as i64);
        }
        // at the end a 'new line' = 10
        self.input.push_back(10);
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
        self.input.pop_front().unwrap()
    }

    fn take_output(&mut self, value: i64) {
        let c = char::from(value as u8);
        match c {
            OUT_FREE | OUT_SCAFFOLD => {
                if !self.debug {
                    self.add_point(c)
                } else {
                    print!("{c}")
                }
            }
            OUT_NEW_LINE => {
                if !self.debug {
                    self.current_location.y += 1;
                    self.current_location.x = -1;
                } else {
                    println!();
                }
            }
            OUT_DIRECTION_UP | OUT_DIRECTION_DOWN | OUT_DIRECTION_LEFT | OUT_DIRECTION_RIGHT => {
                if !self.debug {
                    self.add_point(c)
                } else {
                    print!("{c}")
                }
            }
            _ => {
                self.output = value;
                print!("{c}");
            }
        }
        self.current_location.x += 1;
    }
}

fn main() {
    println!("Hello, day17!");
    if let Some(lines) = io::get_lines("./17.data") {
        let line = &lines[0];
        part_1(&line);
        part_2(&line);
    }
}

fn part_1(line: &str) {
    let mut computer = IntComputer::new(line);
    let mut collector = Collector::new();
    computer.run(&mut collector);
    // collector.print();
    let result = collector.detect_intersections();
    println!("part1 sum of the alignment parameters {}", result);
}

fn part_2(line: &str) {
    let mut computer = IntComputer::new(line);
    computer.change_program(0, 2);
    let mut collector = Collector::new();

    // program coded by hand

    // main
    collector.add_input("A,B,A,C,A,B,C,A,B,C");
    // A
    collector.add_input("R,8,R,10,R,10");
    // B
    collector.add_input("R,4,R,8,R,10,R,12");
    // C
    collector.add_input("R,12,R,4,L,12,L,12");
    // continues video feed
    collector.add_input("y");

    collector.debug = true;
    computer.run(&mut collector);

    println!("part2 result {}", collector.output);

    /*

    R,8,R,10,R,10,R,4,R,8,R,10,R,12,R,8,R,10,R,10,R,12,R,4,L,12,L,12,R,8,R,10,R,10,R,4,R,8,R,10,R,12,R,12,R,4,L,12,L,12,R,8,R,10,R,10,R,4,R,8,R,10,R,12,R,12,R,4,L,12,L,12

     */
}
