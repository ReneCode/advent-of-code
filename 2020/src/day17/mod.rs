// day17

use std::{collections::HashSet, hash::Hash};

use crate::util::io;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Cell {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

pub fn day17() {
    let lines = io::read_lines("17.data").unwrap();

    let mut cube: HashSet<Cell> = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let cell = Cell {
                    x: x as i32,
                    y: y as i32,
                    z: 1,
                    w: 1,
                };
                cube.insert(cell);
            }
        }
    }

    let result_a = part_a(&cube);
    println!("Result A: {}", result_a);
}

fn part_a(start_cube: &HashSet<Cell>) -> i32 {
    let mut cube = start_cube.clone();

    for _ in 0..6 {
        cube = step(&cube);
    }

    cube.len() as i32
}

fn step(cube: &HashSet<Cell>) -> HashSet<Cell> {
    let mut new_cube: HashSet<Cell> = HashSet::new();

    let check_cube = enlarge_cube(&cube);

    for cell in check_cube.iter() {
        let count_active_neighbours = count_active_neighbours(&cube, cell);
        if cube.contains(cell) {
            // active
            if count_active_neighbours == 2 || count_active_neighbours == 3 {
                let new_cell = Cell {
                    x: cell.x,
                    y: cell.y,
                    z: cell.z,
                    w: cell.w,
                };
                new_cube.insert(new_cell);
            }
        } else {
            if count_active_neighbours == 3 {
                let new_cell = Cell {
                    x: cell.x,
                    y: cell.y,
                    z: cell.z,
                    w: cell.w,
                };
                new_cube.insert(new_cell);
            }
        }
    }

    new_cube
}

fn enlarge_cube(cube: &HashSet<Cell>) -> HashSet<Cell> {
    let mut new_cube: HashSet<Cell> = HashSet::new();

    // make a frame around each cell
    for cell in cube.iter() {
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    for w in -1..2 {
                        let neighbour = Cell {
                            x: cell.x + x,
                            y: cell.y + y,
                            z: cell.z + z,
                            w: cell.w + w,
                        };
                        new_cube.insert(neighbour);
                    }
                }
            }
        }
    }

    new_cube
}

fn count_active_neighbours(cube: &HashSet<Cell>, cell: &Cell) -> i32 {
    let mut count = 0;

    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in -1..2 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        // that is me
                        continue;
                    }

                    let neighbour = Cell {
                        x: cell.x + x,
                        y: cell.y + y,
                        z: cell.z + z,
                        w: cell.w + w,
                    };

                    if cube.contains(&neighbour) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
