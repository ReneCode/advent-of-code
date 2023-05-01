use std::collections::{HashMap, VecDeque};

use util::io;
use util::point::Point;

const CELL_ME: char = '@';
const CELL_WALL: char = '#';
const CELL_FREE: char = '.';

fn is_free(c: char) -> bool {
    c == CELL_FREE
}
fn is_key(c: char) -> bool {
    c.is_alphabetic() && c.is_lowercase()
}
fn is_door(c: char) -> bool {
    c.is_alphabetic() && c.is_uppercase()
}
fn is_wall(c: char) -> bool {
    c == CELL_WALL
}
fn door_from_key(c: char) -> Option<char> {
    if !is_key(c) {
        None
    } else {
        Some(c.to_ascii_uppercase())
    }
}

fn main() {
    println!("Hello, day18!");

    if let Some(lines) = io::get_lines("18.data") {
        part_1(&lines);
    }
}

type Coord = (usize, usize);

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<char>>,
    x_len: usize,
    y_len: usize,
    steps: usize,
}

impl Board {
    fn new(lines: &Vec<String>) -> Self {
        let mut cells: Vec<Vec<char>> = Vec::new();
        for row in lines {
            let row: Vec<char> = row.chars().collect();
            cells.push(row)
        }

        Board {
            cells,
            x_len: lines[0].len(),
            y_len: lines.len(),
            steps: 0,
        }
    }

    fn count_keys(&self) -> usize {
        let mut count = 0;
        for row in &self.cells {
            for val in row {
                if is_key(*val) {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_cell(&self, coord: &Coord) -> char {
        self.cells[coord.1][coord.0]
    }

    fn set_cell(&mut self, coord: &Coord, val: char) {
        self.cells[coord.1][coord.0] = val;
    }

    fn get_neigbours(&self, coord: &Coord) -> Vec<Coord> {
        let other: Vec<Coord> = vec![
            (coord.0, coord.1 - 1),
            (coord.0 + 1, coord.1),
            (coord.0, coord.1 + 1),
            (coord.0 - 1, coord.1),
        ];
        let neigbours: Vec<Coord> = other
            .iter()
            .filter(|(col, row)| {
                let val = self.cells[*row][*col];
                let ok = is_free(val) || is_key(val);
                ok
            })
            .map(|coord| *coord)
            .collect();
        neigbours
    }

    fn remove(&mut self, c: char) -> Coord {
        for (idx_row, row) in self.cells.iter_mut().enumerate() {
            for (idx_col, val) in row.iter_mut().enumerate() {
                if c == *val {
                    *val = CELL_FREE;
                    return (idx_col, idx_row);
                }
            }
        }
        (0, 0)
    }
}

fn find_keys(board: &mut Board, start: &Coord, len: usize, result: &mut HashMap<char, usize>) {
    let val = board.get_cell(start);
    if is_key(val) {
        result.insert(val, len);
        // continue finding
        // look also behind a found key
    }

    board.set_cell(start, CELL_ME);

    let neighbours: Vec<Coord> = board
        .get_neigbours(&start)
        .iter()
        .filter(|coord| {
            let val = board.get_cell(coord);
            val == CELL_FREE || is_key(val)
        })
        .map(|coord| *coord)
        .collect();

    for coord in neighbours {
        find_keys(board, &coord, len + 1, result)
    }
}

fn get_connections(lines: &Vec<String>, start: char, visited: &str) -> HashMap<char, usize> {
    let mut board = Board::new(lines);
    for visited_key in visited.chars() {
        board.remove(visited_key);
        if let Some(door) = door_from_key(visited_key) {
            board.remove(door);
        }
    }
    if let Some(door) = door_from_key(start) {
        board.remove(door);
    }
    let start_coord: Coord = board.remove(start);

    let mut results: HashMap<char, usize> = HashMap::new();
    find_keys(&mut board, &start_coord, 0, &mut results);
    results
}

#[derive(Debug)]
struct Ways {
    way_lengths: HashMap<String, usize>,
}

impl Ways {
    fn new() -> Self {
        Ways {
            way_lengths: HashMap::new(),
        }
    }

    fn is_new_or_better(&mut self, keys: &String, end_key: char, len: usize) -> bool {
        let mut way: Vec<char> = keys.chars().collect();
        way.sort();
        way.push(end_key);
        let cache_key = String::from_iter(way.iter());
        if let Some(len_of_way) = self.way_lengths.get_mut(&cache_key) {
            if len < *len_of_way {
                *len_of_way = len;
                true
            } else {
                // println!("remove {cache_key}");
                // same way, but not better
                false
            }
        } else {
            self.way_lengths.insert(cache_key, len);
            true
        }
    }
}

fn part_1(lines: &Vec<String>) {
    let mut keys_list: VecDeque<(String, usize)> = VecDeque::new();
    keys_list.push_back((String::from(CELL_ME), 0));
    let mut ways = Ways::new();

    let mut best_len = usize::MAX;
    let mut best_way = String::new();

    while keys_list.len() > 0 {
        let (keys, prev_len) = keys_list.pop_front().unwrap();

        let start = keys.chars().nth(keys.len() - 1).unwrap();
        let visited = &keys[0..keys.len() - 1];
        let results = get_connections(lines, start, visited);
        if results.len() == 0 {
            println!("finished {visited} {prev_len}");
            if prev_len < best_len {
                best_len = prev_len;
                best_way = String::from(visited)
            }
            // break;
        }
        for (end, len) in results {
            let complete_len = prev_len + len;
            if ways.is_new_or_better(&keys, end, complete_len) {
                let mut new_keys = keys.clone();
                new_keys.push(end);
                println!("{new_keys}  {}", complete_len);
                keys_list.push_back((new_keys, complete_len));
            }
        }
    }

    println!("best way: {best_way} -> {best_len}")
}
