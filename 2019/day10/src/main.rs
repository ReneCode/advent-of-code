#[macro_use]
extern crate assert_float_eq;

use std::{collections::HashSet, f32::consts::PI};

use util::{fraction::Fraction, io::get_lines, point::Point};

fn main() {
    println!("Hello, day10!");
    let points = get_data("./10-example.data");
    part_1(&points);
    part_2(&points);
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

fn calc_max_detect(points: &Vec<Point>) -> (usize, &Point) {
    let mut max_detect = 0;
    let mut max_pt = &points[0];

    for check_point in points {
        let directions = get_directions(points, check_point);
        let detect = directions.len();
        if detect > max_detect {
            max_detect = detect;
            max_pt = check_point;
        }
    }
    (max_detect, max_pt)
}

fn part_1(points: &Vec<Point>) {
    let (max_detect, max_pt) = calc_max_detect(points);
    println!("part-1 max detect {} at {:?}", max_detect, max_pt);
}

fn part_2(points: &Vec<Point>) {
    let (_max_detect, max_pt) = calc_max_detect(points);
    let mut directions = get_directions(points, max_pt);

    let result = 42;
    println!("part-1 result {}", result);
}

fn get_angle((dx, dy): (i32, i32)) -> f32 {
    // up is 0
    // 3 o clock is 1/2 pi
    // 6 o clock is pi
    // 9 o clock is 3/2 pi
    let angle = -(-dx as f32).atan2(dy as f32);
    if angle < 0.0 {
        2.0 * std::f32::consts::PI + angle
    } else {
        angle
    }
}

#[test]
fn test_angle() {
    // up
    assert_eq!(0.0, get_angle((0, 4)));
    assert_eq!(0.0, get_angle((0, 2)));
    // 3 o clock
    assert_eq!(std::f32::consts::FRAC_PI_2, get_angle((2, 0)));
    assert_eq!(std::f32::consts::FRAC_PI_2, get_angle((1, 0)));
    // 6 o clock
    assert_f32_near!(std::f32::consts::PI, get_angle((0, -20)));
    assert_f32_near!(std::f32::consts::PI, get_angle((0, -1)));
    // 9 o clock
    assert_f32_near!(3.0 * std::f32::consts::FRAC_PI_2, get_angle((-3, 0)));
    assert_f32_near!(3.0 * std::f32::consts::FRAC_PI_2, get_angle((-1, 0)));
}

fn get_directions(points: &Vec<Point>, check_pt: &Point) -> HashSet<(i32, i32)> {
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

    directions
}
