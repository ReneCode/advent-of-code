use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::io;

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Wall,
    Start,
    End,
    Empty,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

type PosAndDir = (usize, usize, Direction);

// #[derive(Clone, Debug, PartialEq, Eq, Hash)]
// struct PosAndDir {
//     x: usize,
//     y: usize,
//     dir: Direction,
// }

#[derive(Clone, Debug)]
struct Way {
    positions: Vec<Position>,
}

impl Way {
    fn new(pos: &Position) -> Way {
        Way {
            positions: vec![pos.clone()],
        }
    }

    fn add(&mut self, pos: Position) {
        self.positions.push(pos);
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        self.positions
            .iter()
            .find(|p| p.x == x && p.y == y)
            .is_some()
    }

    fn print(&self, maze: &Maze) {
        for y in 0..maze.maxy {
            for x in 0..maze.maxx {
                if self.contains(x, y) {
                    print!("X");
                } else {
                    print!(
                        "{}",
                        match maze.data[y][x] {
                            Cell::Wall => '#',
                            Cell::Empty => '.',
                            Cell::Start => 'S',
                            Cell::End => 'E',
                        }
                    );
                }
            }
            println!();
        }
    }

    fn calc_cost(&self) -> usize {
        let mut cur_dir = Direction::East;
        let mut cost = 0;
        for (idx, pos) in self.positions.iter().enumerate() {
            if idx == 0 {
                continue;
            }

            let last_pos = &self.positions[idx - 1];
            let dx = pos.x as isize - last_pos.x as isize;
            let dy = pos.y as isize - last_pos.y as isize;

            let new_dir = match (dx, dy) {
                (0, -1) => Direction::North,
                (1, 0) => Direction::East,
                (0, 1) => Direction::South,
                (-1, 0) => Direction::West,
                _ => panic!("Invalid direction"),
            };

            match (cur_dir, new_dir) {
                (Direction::North | Direction::South, Direction::West | Direction::East) => {
                    cost += 1000;
                    cost += 1;
                }
                (Direction::West | Direction::East, Direction::North | Direction::South) => {
                    cost += 1000;
                    cost += 1;
                }
                (cur, new) => {
                    if cur == new {
                        cost += 1;
                    } else {
                        panic!("Invalid direction change {:?} -> {:?}", cur, new);
                    }
                }
            }

            cur_dir = new_dir;
        }

        cost
    }
}

struct Maze {
    data: Vec<Vec<Cell>>,
    maxx: usize,
    maxy: usize,
}

impl Maze {
    fn from(lines: Vec<String>) -> Maze {
        let maxx = lines[0].len();
        let maxy = lines.len();
        let mut data = vec![vec![Cell::Empty; maxx]; maxy];

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                data[y][x] = match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Empty,
                    'S' => Cell::Start,
                    'E' => Cell::End,
                    _ => panic!("Invalid character in maze"),
                };
            }
        }

        Maze { data, maxx, maxy }
    }

    fn print(&self) {
        for y in 0..self.maxy {
            for x in 0..self.maxx {
                print!(
                    "{}",
                    match self.data[y][x] {
                        Cell::Wall => '#',
                        Cell::Empty => '.',
                        Cell::Start => 'S',
                        Cell::End => 'E',
                    }
                );
            }
            println!();
        }
    }

    fn find_start(&self) -> Position {
        for y in 0..self.maxy {
            for x in 0..self.maxx {
                if let Cell::Start = self.data[y][x] {
                    return Position::new(x, y);
                }
            }
        }
        panic!("No start found");
    }

    fn find_end(&self) -> Position {
        for y in 0..self.maxy {
            for x in 0..self.maxx {
                if let Cell::End = self.data[y][x] {
                    return Position::new(x, y);
                }
            }
        }
        panic!("No end found");
    }

    fn get_neighbours(&self, pos_and_dir: PosAndDir) -> Vec<(usize, usize, Direction)> {
        let mut neighbours = Vec::new();
        let (x, y, dir) = pos_and_dir;
        if x > 0 && dir != Direction::East {
            if self.data[y][x - 1] == Cell::Empty || self.data[y][x - 1] == Cell::End {
                neighbours.push((x - 1, y, Direction::West));
            }
        }
        if x < self.maxx - 1 && dir != Direction::West {
            if self.data[y][x + 1] == Cell::Empty || self.data[y][x + 1] == Cell::End {
                neighbours.push((x + 1, y, Direction::East));
            }
        }
        if y > 0 && dir != Direction::South {
            if self.data[y - 1][x] == Cell::Empty || self.data[y - 1][x] == Cell::End {
                neighbours.push((x, y - 1, Direction::North));
            }
        }
        if y < self.maxy - 1 && dir != Direction::North {
            if self.data[y + 1][x] == Cell::Empty || self.data[y + 1][x] == Cell::End {
                neighbours.push((x, y + 1, Direction::South));
            }
        }

        neighbours
    }

    fn bfs(&self) {
        let mut queue = Vec::new();
        let start = self.find_start();
        queue.push((start.x, start.y, Direction::East));

        let mut visited = HashSet::new();
        visited.insert((start.x, start.y, Direction::East));

        let mut prev = HashMap::new();

        while !queue.is_empty() {
            let cur_pos_and_dir = queue.remove(0);
            let neighbours = self.get_neighbours(cur_pos_and_dir);

            for next_pos_and_dir in neighbours {
                if !visited.contains(&next_pos_and_dir) {
                    queue.push(next_pos_and_dir);
                    visited.insert(next_pos_and_dir);
                    prev.insert(next_pos_and_dir, cur_pos_and_dir);
                }
            }
        }

        // now prev is filled with the shortest path
    }

    fn dijstra(&self) -> usize {
        let mut queue = Vec::new();
        let start = self.find_start();
        queue.push((start.x, start.y, Direction::East));

        let mut prev = HashMap::new();
        let mut distance = HashMap::new();

        distance.insert((start.x, start.y, Direction::East), 0);

        while !queue.is_empty() {
            // find q with min distance
            let a = queue
                .iter()
                .enumerate()
                .map(|(idx, q)| (idx, distance.get(q).unwrap()))
                .min_by_key(|x| x.1)
                .unwrap();
            let cur_pos_and_dir = queue.remove(a.0);

            let neighbours = self.get_neighbours(cur_pos_and_dir);

            let cur_direction = cur_pos_and_dir.2;
            let cur_dist = *distance.get(&cur_pos_and_dir).unwrap();
            for next_pos_and_dir in neighbours {
                let mut alt = cur_dist;
                match (cur_direction, next_pos_and_dir.2) {
                    (Direction::North | Direction::South, Direction::West | Direction::East) => {
                        alt += 1000;
                        alt += 1;
                    }
                    (Direction::West | Direction::East, Direction::North | Direction::South) => {
                        alt += 1000;
                        alt += 1;
                    }
                    (cur, new) => {
                        if cur == new {
                            alt += 1;
                        } else {
                            panic!("Invalid direction change {:?} -> {:?}", cur, new);
                        }
                    }
                }
                if alt < *distance.get(&next_pos_and_dir).unwrap_or(&std::usize::MAX) {
                    distance.insert(next_pos_and_dir, alt);
                    prev.insert(next_pos_and_dir, cur_pos_and_dir);
                    queue.push(next_pos_and_dir);
                }
            }
        }

        // now prev is filled with the shortest path

        let end = self.find_end();

        let a1 = distance
            .get(&(end.x, end.y, Direction::North))
            .or(Some(&usize::MAX))
            .unwrap();
        let a2 = distance
            .get(&(end.x, end.y, Direction::West))
            .or(Some(&usize::MAX))
            .unwrap();
        let a3 = distance
            .get(&(end.x, end.y, Direction::East))
            .or(Some(&usize::MAX))
            .unwrap();
        let a4 = distance
            .get(&(end.x, end.y, Direction::South))
            .or(Some(&usize::MAX))
            .unwrap();

        let a = a1.min(a2.min(a3.min(a4)));
        *a
    }
}

pub fn day16() {
    let _lines = io::read_lines("./src/day16/16.data").unwrap();

    let maze = Maze::from(_lines);

    let a = maze.dijstra();
    maze.print();

    println!("Day16 part 1: {:?}", a);
}
