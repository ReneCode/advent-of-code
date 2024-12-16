use crate::util::io;

#[derive(Clone)]
enum Cell {
    Wall,
    Start,
    End,
    Empty,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

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

    fn find_all_ways(&self) -> Vec<Way> {
        let mut all_complete_ways = Vec::new();

        let start = self.find_start();
        let way = Way::new(&start);
        let mut ways = vec![way];

        while !ways.is_empty() {
            let way = ways.remove(0);
            let last_pos = way.positions.last().unwrap();
            let x = last_pos.x;
            let y = last_pos.y;

            if let Cell::End = self.data[y][x] {
                all_complete_ways.push(way);
                continue;
            }

            if x > 0 {
                match self.data[y][x - 1] {
                    Cell::Empty | Cell::End => {
                        if !way.contains(x - 1, y) {
                            let mut new_way = way.clone();
                            new_way.add(Position::new(x - 1, y));
                            ways.push(new_way);
                        }
                    }
                    _ => {}
                }
            }
            if x < self.maxx - 1 {
                match self.data[y][x + 1] {
                    Cell::Empty | Cell::End => {
                        if !way.contains(x + 1, y) {
                            let mut new_way = way.clone();
                            new_way.add(Position::new(x + 1, y));
                            ways.push(new_way);
                        }
                    }
                    _ => {}
                }
            }
            if y > 0 {
                match self.data[y - 1][x] {
                    Cell::Empty | Cell::End => {
                        if !way.contains(x, y - 1) {
                            let mut new_way = way.clone();
                            new_way.add(Position::new(x, y - 1));
                            ways.push(new_way);
                        }
                    }
                    _ => {}
                }
            }
            if y < self.maxy - 1 {
                match self.data[y + 1][x] {
                    Cell::Empty | Cell::End => {
                        if !way.contains(x, y + 1) {
                            let mut new_way = way.clone();
                            new_way.add(Position::new(x, y + 1));
                            ways.push(new_way);
                        }
                    }
                    _ => {}
                }
            }
        }
        all_complete_ways
    }
}

pub fn day16() {
    let _lines = io::read_lines("./src/day16/16.data").unwrap();

    let maze = Maze::from(_lines);

    // maze.print();

    let ways = maze.find_all_ways();

    let mut min_cost = std::usize::MAX;
    for way in ways {
        let cost = way.calc_cost();
        min_cost = min_cost.min(cost);
        println!("--------- Cost: {}  len:{}", cost, way.positions.len());
        // way.print(&maze);
    }

    println!("Day16 part 1: {:?}", min_cost);
}
