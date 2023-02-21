use std::collections::HashSet;

use util::{fraction::Fraction, io::get_lines, point::Point};

fn main() {
    println!("Hello, day10!");
    let points = get_data("./10.data");
    part_1(&points);
}

fn get_data(filename: &str) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    if let Some(lines) = get_lines(filename) {
        let y_count = lines.len();
        let x_count = lines[0].len();
        for y in 0..y_count {
            for x in 0..x_count {
                let c = lines[y].chars().nth(x).unwrap();
                if c == '#' {
                    result.push(Point::new(x as i32, y as i32))
                }
            }
        }
    }
    result
}

fn part_1(points: &Vec<Point>) {
    let mut max_detect = 0;

    for check_point in points {
        let detect = detect_from(points, check_point);
        // println!("{:?} / {}", check_point, detect);
        max_detect = max_detect.max(detect);
    }

    println!("part-1 max detect {}", max_detect);
}

fn detect_from(points: &Vec<Point>, check_pt: &Point) -> usize {
    let mut directions: HashSet<(i32, i32)> = HashSet::new();
    for pt in points {
        if pt.equal(check_pt) {
            continue;
        }
        let mut diff = pt.sub(check_pt);
        if diff.x == 0 {
            diff.y = diff.y.signum();
        }
        if diff.y == 0 {
            diff.x = diff.x.signum();
        }
        let fraction = Fraction::new(diff.x, diff.y).reduce();

        directions.insert((fraction.numerator, fraction.denominator));
    }

    directions.len()
}
