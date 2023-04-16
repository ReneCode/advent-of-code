use std::collections::HashMap;

use util::io;
use util::point::{BoundingBox, Point};

const CELL_ME: char = '@';
const CELL_WALL: char = '#';
const CELL_FREE: char = '.';

fn is_key(c: char) -> bool {
    c.is_alphabetic() && c.is_lowercase()
}
fn is_door(c: char) -> bool {
    c.is_alphabetic() && c.is_uppercase()
}
fn is_wall(c: char) -> bool {
    c == CELL_WALL
}
fn door_from_key(c: char) -> char {
    if !is_key(c) {
        panic!("no key")
    }
    c.to_ascii_uppercase()
}

fn main() {
    println!("Hello, day18!");

    if let Some(lines) = io::get_lines("18-example.data") {
        part_1(&lines);
    }
}

struct Board {
    cells: HashMap<Point, char>,
    x_len: usize,
    y_len: usize,
}

impl Board {
    fn new(lines: &Vec<String>) -> Self {
        let mut cells: HashMap<Point, char> = HashMap::new();
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                let pt = Point::new(x as i32, y as i32);
                cells.insert(pt, c);
            }
        }

        Board {
            cells,
            x_len: lines[0].len(),
            y_len: lines.len(),
        }
    }

    fn find_start(&self) -> Point {
        for (pt, c) in self.cells.iter() {
            if *c == CELL_ME {
                return pt.clone();
            }
        }
        panic!("start not found")
    }

    fn get_neigbours(&self, pos: &Point) -> Vec<Point> {
        let other = [
            Point::new(pos.x, pos.y - 1),
            Point::new(pos.x + 1, pos.y),
            Point::new(pos.x, pos.y + 1),
            Point::new(pos.x - 1, pos.y),
        ];
        let neigbours: Vec<Point> = other
            .iter()
            .filter(|pt| {
                let c = self.cells.get(pt).unwrap();

                let is_blocked = is_wall(*c) || is_door(*c);
                !is_blocked
            })
            .map(|pt| pt.clone())
            .collect();
        neigbours
    }

    fn remove_me(&mut self) -> Point {
        for (pt, val) in self.cells.iter_mut() {
            if *val == CELL_ME {
                *val = CELL_FREE;
                return pt.clone();
            }
        }
        panic!("me not found")
    }

    fn find_connections(&self, connections: &mut Vec<Connection>, way: &mut Way) {
        let me = way.last_point().clone();

        let cell = self.cells.get(&me).unwrap();
        if is_key(*cell) {
            way.add_key(*cell);

            let start = way.first_point();
            let end = me;
            let connection = Connection::new(*start, end, way.len(), *cell);
            connections.push(connection);
        } else {
            let neigbours: Vec<Point> = self
                .get_neigbours(&me)
                .iter()
                .filter(|pt| !way.contains_point(pt))
                .map(|pt| *pt)
                .collect();
            if neigbours.len() == 1 {
                let pt = neigbours[0];
                way.add_point(pt);
                self.find_connections(connections, way);
            } else {
                for (idx, pt) in neigbours.iter().enumerate() {
                    if (idx == 0) {
                        way.add_point(*pt);
                        self.find_connections(connections, way);
                    } else {
                        let mut new_way = way.clone();
                        new_way.add_point(*pt);
                        self.find_connections(connections, &mut new_way);
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Way {
    points: Vec<Point>,
    keys: Vec<char>,
    steps: usize,
    all_steps: usize,
}
impl Clone for Way {
    fn clone(&self) -> Self {
        Self {
            points: self.points.clone(),
            keys: self.keys.clone(),
            steps: self.steps,
            all_steps: self.all_steps,
        }
    }
}
impl Way {
    fn new() -> Self {
        Way {
            points: Vec::new(),
            keys: Vec::new(),
            steps: 0,
            all_steps: 0,
        }
    }
    fn add_point(&mut self, pt: Point) {
        self.points.push(pt);
        self.steps += 1;
    }
    fn last_point(&self) -> &Point {
        &self.points[self.points.len() - 1]
    }
    fn first_point(&self) -> &Point {
        &self.points[0]
    }
    fn len(&self) -> usize {
        self.points.len()
    }

    fn contains_point(&self, pt: &Point) -> bool {
        self.points.contains(pt)
    }
    fn add_key(&mut self, key: char) {
        self.keys.push(key);
        self.all_steps += self.steps;
        self.steps = 0
    }
}

#[derive(Debug)]
struct Connection {
    start: Point,
    end: Point,
    len: usize,
    key: char,
}
impl Connection {
    fn new(start: Point, end: Point, len: usize, key: char) -> Self {
        Connection {
            start,
            end,
            len,
            key,
        }
    }
}

fn part_1(lines: &Vec<String>) {
    let mut board = Board::new(lines);
    let me = board.remove_me();

    let mut way = Way::new();
    way.add_point(me);

    let mut connections: Vec<Connection> = Vec::new();

    board.find_connections(&mut connections, &mut way);

    println!("{:?}", connections);
}
