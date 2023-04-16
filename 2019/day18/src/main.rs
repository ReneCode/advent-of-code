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

#[derive(clone)]
struct Way {
    keys: Vec<char>,
    coords: Vec<Coord>,
    steps: usize,
}
impl Way {
    fn new() -> Self {
        Way {
            keys: Vec::new(),
            coords: Vec::new(),
            steps: 0,
        }
    }

    fn add(coord: &Coord, c: char) {}
}

fn find_way(board: &mut Board, way: &mut Vec<Coord>) {
    let last_coord = way[way.len() - 1];
    let val = board.cells[last_coord.1][last_coord.0];

    if is_key(val) {
        print!("({val})");
        board.remove(val);
        let door = door_from_key(val);
        board.remove(door);
        way.clear();
        way.push(last_coord);

        if board.count_keys() == 0 {
            println!(" finished all keys collected {:?} {}", way, board.steps)
        }
    }

    let neighbours: Vec<Coord> = board
        .get_neigbours(&last_coord)
        .iter()
        .filter(|coord| !way.contains(coord))
        .map(|coord| *coord)
        .collect();

    if neighbours.len() == 0 {
        println!(" can't go on.")
    }

    for (idx, coord) in neighbours.iter().enumerate() {
        if idx == 0 {
            way.push(*coord);
            board.steps += 1;
            find_way(board, way)
        } else {
            let mut new_way = way.clone();
            new_way.push(*coord);
            board.steps += 1;
            let mut new_board = board.clone();
            println!();
            find_way(&mut new_board, &mut new_way);
        }
    }
}

fn part_1(lines: &Vec<String>) {
    let mut board = Board::new(lines);
    let start: Coord = board.remove(CELL_ME);

    let mut way: Vec<Coord> = Vec::new();
    way.push(start);
    find_way(&mut board, &mut way);

    // println!("{:?}", connections);
}
