// day10

use core::num;
use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{io, math, parse};

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    x: i32,
    y: i32,
    shape: char,
}

impl Tile {
    fn new(x: i32, y: i32, shape: char) -> Self {
        Tile {
            x: x,
            y: y,
            shape: shape,
        }
    }

    fn directions(&self) -> Vec<Direction> {
        match self.shape {
            'S' => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            '|' => vec![Direction::North, Direction::South],
            '-' => vec![Direction::West, Direction::East],
            'L' => vec![Direction::North, Direction::East],
            'J' => vec![Direction::North, Direction::West],
            'F' => vec![Direction::South, Direction::East],
            '7' => vec![Direction::South, Direction::West],
            '.' => vec![],
            _ => panic!("bad shape - no valid directions"),
        }
    }
}

struct Area {
    tiles: Vec<Vec<Tile>>,
    x_len: i32,
    y_len: i32,
}

impl Area {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        Area {
            y_len: tiles.len() as i32,
            x_len: tiles[0].len() as i32,
            tiles: tiles,
        }
    }
    fn find_tile<'a>(&'a self, c: char) -> Option<&'a Tile> {
        for row in self.tiles.iter() {
            for tile in row {
                if tile.shape == c {
                    return Some(tile);
                }
            }
        }
        None
    }

    fn get_loop<'a>(&'a self, start_tile: &'a Tile) -> Vec<&'a Tile> {
        let mut result: Vec<&Tile> = Vec::new();
        result.push(start_tile);
        let mut tile = start_tile;
        loop {
            let next_tiles = self.get_next_tiles(&tile);
            if let Some(next_tile) = next_tiles
                .iter()
                .find(|t| !result.contains(t) && t.shape != '.')
            {
                result.push(next_tile);
                tile = next_tile;
            } else {
                break;
            }
        }

        result
    }

    fn get_tile(&self, x: i32, y: i32) -> &Tile {
        self.tiles[y as usize].iter().nth(x as usize).unwrap()
    }

    fn get_neighbour_tile<'a>(&'a self, tile: &Tile, direction: &Direction) -> Option<&'a Tile> {
        let x = tile.x;
        let y = tile.y;
        match direction {
            Direction::North => {
                if y > 0 {
                    let t = self.get_tile(x, y - 1);
                    if t.directions().contains(&Direction::South) {
                        return Some(t);
                    }
                }
            }
            Direction::East => {
                if x + 1 < self.x_len {
                    let t = self.get_tile(x + 1, y);
                    if t.directions().contains(&Direction::West) {
                        return Some(t);
                    }
                }
            }
            Direction::South => {
                if y + 1 < self.y_len {
                    let t = self.get_tile(x, y + 1);
                    if t.directions().contains(&Direction::North) {
                        return Some(t);
                    }
                }
            }
            Direction::West => {
                if x > 0 {
                    let t = self.get_tile(x - 1, y);
                    if t.directions().contains(&Direction::East) {
                        return Some(t);
                    }
                }
            }
        }
        None
    }

    fn get_next_tiles(&self, start_tile: &Tile) -> Vec<&Tile> {
        let mut result: Vec<&Tile> = Vec::new();

        let next_tiles = start_tile
            .directions()
            .iter()
            .map(|d| self.get_neighbour_tile(start_tile, d))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .filter(|t| !result.contains(t))
            .collect_vec();

        for next_tile in next_tiles {
            // let oposite_directions = next_tile
            //     .directions()
            //     .iter()
            //     .map(|d| oposite_direction(d))
            //     .collect_vec();

            // let direction_fit = start_tile
            //     .directions()
            //     .iter()
            //     .any(|d| oposite_directions.contains(d));

            // // .v.directions.iter().any(|d| oposite_directions.contains(d));
            // if direction_fit {
            result.push(next_tile);
            // }
        }
        result
    }
}

fn oposite_direction(d: &Direction) -> Direction {
    let oposite = match d {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    };
    oposite
}

pub fn day10() {
    println!("hello day10");

    let lines = io::read_lines("./src/day10/10.data").unwrap();

    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    for (y_idx, line) in lines.iter().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (x_idx, shape) in line.chars().enumerate() {
            let tile = Tile::new(x_idx as i32, y_idx as i32, shape);
            row.push(tile)
        }
        tiles.push(row);
    }
    let area = Area::new(tiles);
    let start_tile = area.find_tile('S').unwrap();
    let tile_loop = area.get_loop(&start_tile);

    // println!("{:?}", tile_loop);
    let result_a = tile_loop.len() / 2;
    println!("Result A: {result_a}");
}
