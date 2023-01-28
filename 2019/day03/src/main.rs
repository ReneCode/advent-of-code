use std::collections::{HashMap, HashSet};

extern crate util;

struct BoundingBox {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl BoundingBox {
    fn new(x: i32, y: i32) -> BoundingBox {
        BoundingBox {
            x_min: x,
            x_max: x,
            y_min: y,
            y_max: y,
        }
    }
    fn add(&mut self, x: i32, y: i32) {
        self.x_min = self.x_min.min(x);
        self.x_max = self.x_max.max(x);
        self.y_min = self.y_min.min(y);
        self.y_max = self.y_max.max(y);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn clone(&self) -> Self {
        Point::new(self.x, self.y)
    }
}

struct Line {
    p1: Point,
    p2: Point,
}

fn between(a: i32, r1: i32, r2: i32) -> bool {
    (r1 < a && a < r2) || (r2 < a && a < r1)
}

impl Line {
    fn new(p1: &Point, p2: &Point) -> Line {
        Line {
            p1: p1.clone(),
            p2: p2.clone(),
        }
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn intersect(&self, other: &Line) -> Option<Point> {
        if self.is_horizontal()
            && other.is_vertical()
            && between(other.p1.x, self.p1.x, self.p2.x)
            && between(self.p1.y, other.p1.y, other.p2.y)
        {
            return Some(Point::new(other.p1.x, self.p1.y));
        }
        if self.is_vertical()
            && other.is_horizontal()
            && between(self.p1.x, other.p1.x, other.p2.x)
            && between(other.p1.y, self.p1.y, self.p2.y)
        {
            return Some(Point::new(self.p1.x, other.p1.y));
        }
        None
    }

    fn is_on_line(&self, point: &Point) -> bool {
        if self.is_horizontal() {
            return point.y == self.p1.y && between(point.x, self.p1.x, self.p2.x);
        }
        if self.is_vertical() {
            return point.x == self.p1.x && between(point.y, self.p1.y, self.p2.y);
        }
        return false;
    }

    fn steps(&self) -> i32 {
        if self.is_horizontal() {
            return (self.p1.x - self.p2.x).abs();
        }
        if self.is_vertical() {
            return (self.p1.y - self.p2.y).abs();
        }
        panic!("can't calc steps");
    }

    fn steps_to_reach(&self, point: &Point) -> i32 {
        if self.is_horizontal() {
            return (point.x - self.p1.x).abs();
        }
        if self.is_vertical() {
            return (point.y - self.p1.y).abs();
        }
        panic!("can't calc steps_to_reach");
    }
}

struct Board {
    first_lines: Vec<Line>,
    second_lines: Vec<Line>,
}

fn convert_to_points(path: &str) -> Vec<Point> {
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = 0;
    points.push(Point { x, y });
    for cmd in path.split(",").map(|a| a.to_string()) {
        let val = cmd.get(1..).unwrap().parse::<i32>().unwrap();
        match cmd.get(0..1) {
            Some("U") => {
                y += val;
            }
            Some("D") => {
                y -= val;
            }
            Some("L") => {
                x -= val;
            }
            Some("R") => {
                x += val;
            }

            _ => {
                println!("ups {cmd}")
            }
        }
        points.push(Point { x, y });

        // println!("{dir} / {val}");
    }
    points
}

fn get_data(filename: &str) -> Option<Board> {
    if let Some(input) = util::io::get_lines(filename) {
        let first_points = convert_to_points(&input[0]);
        let second_points = convert_to_points(&input[1]);
        let mut bounding_box = BoundingBox::new(0, 0);
        for point in first_points.iter() {
            bounding_box.add(point.x, point.y);
        }
        for point in second_points.iter() {
            bounding_box.add(point.x, point.y);
        }

        let first_lines: Vec<Line> = first_points
            .windows(2)
            .map(|window| Line::new(&window[0], &window[1]))
            .collect();

        let second_lines: Vec<Line> = second_points
            .windows(2)
            .map(|points| Line::new(&points[0], &points[1]))
            .collect();

        let data = Board {
            first_lines,
            second_lines,
        };
        Some(data)
    } else {
        None
    }
}

fn main() {
    println!("Hello, day03!");
    if let Some(board) = get_data("./03.data") {
        part_1(&board);
        part_2(&board);
    }
}

fn get_intersections(lines_a: &Vec<Line>, lines_b: &Vec<Line>) -> Vec<Point> {
    let mut intersections = Vec::new();
    for first_line in lines_a.iter() {
        for second_line in lines_b.iter() {
            if let Some(intersect) = first_line.intersect(&second_line) {
                intersections.push(intersect)
            }
        }
    }
    intersections
}

fn part_1(board: &Board) {
    let intersections = get_intersections(&board.first_lines, &board.second_lines);

    let mut distances: Vec<i32> = intersections
        .iter()
        .map(|p| p.x.abs() + p.y.abs())
        .collect();
    distances.sort();

    println!("part1 min distance {}", distances[0]);
}

fn count_steps_to_point(lines: &Vec<Line>, point: &Point) -> i32 {
    let mut steps = 0;
    for line in lines {
        if !line.is_on_line(point) {
            steps += line.steps();
        } else {
            steps += line.steps_to_reach(point);

            break;
        }
    }
    steps
}

fn part_2(board: &Board) {
    let intersections = get_intersections(&board.first_lines, &board.second_lines);

    let mut steps: Vec<i32> = intersections
        .iter()
        .map(|point| {
            count_steps_to_point(&board.first_lines, point)
                + count_steps_to_point(&board.second_lines, point)
        })
        .collect();

    steps.sort();
    println!("part2 min steps {}", steps[0]);
}
