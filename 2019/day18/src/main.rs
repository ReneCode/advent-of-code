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

    // fn find_connections(&self, connections: &mut Vec<Connection>, way: &mut Way) {
    //     let me = way.last_point().clone();

    //     let cell = self.cells.get(&me).unwrap();
    //     if is_key(*cell) {
    //         way.add_key(*cell);

    //         let start = way.first_point();
    //         let end = me;
    //         let connection = Connection::new(*start, end, way.len(), *cell);
    //         connections.push(connection);
    //     } else {
    //         let neigbours: Vec<Point> = self
    //             .get_neigbours(&me)
    //             .iter()
    //             .filter(|pt| !way.contains_point(pt))
    //             .map(|pt| *pt)
    //             .collect();
    //         if neigbours.len() == 1 {
    //             let pt = neigbours[0];
    //             way.add_point(pt);
    //             self.find_connections(connections, way);
    //         } else {
    //             for (idx, pt) in neigbours.iter().enumerate() {
    //                 if (idx == 0) {
    //                     way.add_point(*pt);
    //                     self.find_connections(connections, way);
    //                 } else {
    //                     let mut new_way = way.clone();
    //                     new_way.add_point(*pt);
    //                     self.find_connections(connections, &mut new_way);
    //                 }
    //             }
    //         }
    //     }
    // }
}

#[derive(Clone, Debug)]
struct Way {
    keys: Vec<char>,
    all_coords: Vec<Coord>,
    last_coords: Vec<Coord>,
    steps: usize,
}
impl Way {
    fn new() -> Self {
        Way {
            keys: Vec::new(),
            all_coords: Vec::new(),
            last_coords: Vec::new(),
            steps: 0,
        }
    }

    fn last_coord(&self) -> Coord {
        self.last_coords[self.last_coords.len() - 1]
    }

    fn add_coord(&mut self, coord: Coord) {
        self.steps += 1;
        self.all_coords.push(coord);
        self.last_coords.push(coord);
    }
    fn add_key(&mut self, c: char) {
        self.keys.push(c);
        let last_coord = self.last_coord();
        self.last_coords.clear();
        self.last_coords.push(last_coord);
    }

    fn contains_coord(&self, coord: &Coord) -> bool {
        self.last_coords.contains(coord)
    }
}

fn find_way(board: &mut Board, way: &mut Way, results: &mut Vec<usize>) {
    let last_coord = way.last_coord();
    let val = board.cells[last_coord.1][last_coord.0];

    if is_key(val) {
        way.add_key(val);
        board.remove(val);
        let door = door_from_key(val);
        board.remove(door);

        if board.count_keys() == 0 {
            // println!(" finished all keys collected {:?}", way);
            results.push(way.steps - 1);
            return;
        }
    }

    let neighbours: Vec<Coord> = board
        .get_neigbours(&last_coord)
        .iter()
        .filter(|coord| !way.contains_coord(coord))
        .map(|coord| *coord)
        .collect();

    // if neighbours.len() == 0 {
    //     println!(" can't go on.")
    // }

    for (idx, coord) in neighbours.iter().enumerate() {
        let mut new_way = way.clone();
        new_way.add_coord(*coord);
        let mut new_board = board.clone();
        find_way(&mut new_board, &mut new_way, results);
    }
}

fn part_1(lines: &Vec<String>) {
    let mut board = Board::new(lines);
    let start: Coord = board.remove(CELL_ME);

    let mut way = Way::new();
    way.add_coord(start);

    let mut results: Vec<usize> = Vec::new();
    find_way(&mut board, &mut way, &mut results);

    println!("{:?}", results);
}
