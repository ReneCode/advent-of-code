// day18

use itertools::Itertools;

const EMPTY: char = '.';
const HOLE: char = '#';

use crate::util::{
    io,
    matrix::{calc_direction, Direction, Matrix, Position},
    parse,
};

pub fn day18() {
    println!("hello day18");

    let lines = io::read_lines("./src/day18/18.data").unwrap();

    part_a(&lines);

    part_b(&lines);
}

fn color_to_dir_len(color: &str) -> (Direction, i64) {
    let hex = &color[0..5];
    let last_char = &color[5..6].chars().nth(0).unwrap();

    let len = i64::from_str_radix(hex, 16).unwrap();
    let dir = match last_char {
        '0' => Direction::RIGHT,
        '1' => Direction::DOWN,
        '2' => Direction::LEFT,
        '3' => Direction::UP,
        _ => panic!("bad direction {last_char}"),
    };
    return (dir, len);
}

fn part_b(lines: &Vec<String>) {
    let commands = lines
        .iter()
        .map(|line| {
            let tok = parse::to_str(line, ' ');
            let third = &tok[2];
            let color = &third[2..=7];
            return color_to_dir_len(color);
        })
        .collect_vec();

    let mut points: Vec<(i64, i64)> = Vec::new();
    let mut pt: (i64, i64) = (0, 0);
    points.push(pt);
    let mut border_len: i64 = 0;
    for (dir, count) in commands {
        border_len += count;
        match dir {
            Direction::UP => {
                pt = (pt.0, pt.1 - count);
                points.push(pt)
            }
            Direction::RIGHT => {
                pt = (pt.0 + count, pt.1);
                points.push(pt)
            }
            Direction::DOWN => {
                pt = (pt.0, pt.1 + count);
                points.push(pt)
            }
            Direction::LEFT => {
                pt = (pt.0 - count, pt.1);
                points.push(pt)
            }

            _ => panic!("bad direction {:?}", dir),
        }
    }

    // area of polygon
    // https://en.wikipedia.org/wiki/Shoelace_formula

    let mut sum_det = 0;
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];
        let det = p1.0 * p2.1 - p2.0 * p1.1;
        sum_det += det;
    }

    // take care of the border
    let result_b = sum_det / 2 + border_len / 2 + 1;
    println!("Result B: {result_b}");

    // println!("{:?}", points);
}

fn part_a(lines: &Vec<String>) {
    let commands = lines
        .iter()
        .map(|line| {
            let tok = parse::to_str(line, ' ');
            let dir = tok[0].chars().nth(0).unwrap();
            let len: usize = tok[1].parse().unwrap();
            return (dir, len);
        })
        .collect_vec();

    let mut holes: Vec<Position> = Vec::new();

    // positions are usize, but it can be that we dig into negative direction
    let offset = (1000, 1000);
    let mut current_pos = offset;
    holes.push(current_pos);
    for (command, count) in commands {
        match command {
            'U' => {
                for i in 0..count {
                    current_pos.1 -= 1;
                    holes.push(current_pos)
                }
            }
            'D' => {
                for i in 0..count {
                    current_pos.1 += 1;
                    holes.push(current_pos)
                }
            }
            'R' => {
                for i in 0..count {
                    current_pos.0 += 1;
                    holes.push(current_pos)
                }
            }
            'L' => {
                for i in 0..count {
                    current_pos.0 -= 1;
                    holes.push(current_pos)
                }
            }

            _ => panic!("bad command"),
        }
    }
    let min_x = holes.iter().map(|p| p.0).min().unwrap();
    let max_x = holes.iter().map(|p| p.0).max().unwrap();
    let min_y = holes.iter().map(|p| p.1).min().unwrap();
    let max_y = holes.iter().map(|p| p.1).max().unwrap();

    // normalize area to start on index (0,0)
    let mut area = Matrix::new(max_x - min_x + 1, max_y - min_y + 1, EMPTY);
    for (x, y) in holes {
        area.set((x - min_x, y - min_y), HOLE);
    }

    let mut start_pos = (0, 1);
    for x in 0..area.xlen() {
        if area.get((x, 1)) == &HOLE {
            start_pos.0 = x + 1;
            break;
        }
    }
    flood_fill(&mut area, start_pos);

    let result_a = area
        .all_positions()
        .iter()
        .filter(|p| area.get(**p) == &HOLE)
        .count();

    println!("Result A: {result_a}");
}

fn flood_fill(area: &mut Matrix<char>, pos: Position) {
    if *area.get(pos) == HOLE {
        return;
    }

    area.set(pos, HOLE);

    for dir in Direction::into_iter() {
        if let Some(next_pos) = area.next_pos(pos, dir) {
            flood_fill(area, next_pos);
        }
    }
}
