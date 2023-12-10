// day10

use core::num;
use std::{collections::HashMap, ops::IndexMut};

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
        get_directions(self.shape)
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

fn get_directions(shape: char) -> Vec<Direction> {
    match shape {
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

    let result_b: i32 = part_2(&area, &tile_loop);
    println!("Result B: {result_b}");
}

fn get_start_shape(tiles: &Vec<&Tile>) -> char {
    let mut directions: Vec<Direction> = Vec::new();
    directions.push(get_delta_direction(tiles[0], tiles[1]));
    directions.push(get_delta_direction(tiles[0], tiles[tiles.len() - 1]));

    let mut shape = '.';
    if directions.contains(&Direction::North) && directions.contains(&Direction::South) {
        shape = '|'
    }
    if directions.contains(&Direction::West) && directions.contains(&Direction::East) {
        shape = '-'
    }
    if directions.contains(&Direction::North) && directions.contains(&Direction::East) {
        shape = 'L'
    }
    if directions.contains(&Direction::South) && directions.contains(&Direction::East) {
        shape = 'F'
    }
    if directions.contains(&Direction::North) && directions.contains(&Direction::West) {
        shape = 'J'
    }
    if directions.contains(&Direction::South) && directions.contains(&Direction::West) {
        shape = '7'
    }
    shape
}

fn get_delta_direction(t1: &Tile, t2: &Tile) -> Direction {
    let dx = t2.x - t1.x;
    if dx > 0 {
        return Direction::East;
    }
    if dx < 0 {
        return Direction::West;
    }
    let dy = t2.y - t1.y;
    if dy > 0 {
        return Direction::South;
    }
    if dy < 0 {
        return Direction::North;
    }

    panic!("no valid delta direction")
}

fn part_2(area: &Area, tiles: &Vec<&Tile>) -> i32 {
    let mut result = 0;
    for y in 0..area.y_len {
        let mut count = 0;
        let mut start_east = false;
        let mut start_east_shape = ' ';
        for x in 0..area.x_len {
            if let Some(tile) = tiles.iter().find(|t| t.x == x && t.y == y) {
                let mut shape = tile.shape;

                if shape == 'S' {
                    shape = get_start_shape(tiles);
                }

                if shape == '-' {
                    continue;
                }
                if shape == '|' {
                    count += 1;
                    continue;
                }
                let with_west = get_directions(shape).contains(&Direction::West);
                let with_east = get_directions(shape).contains(&Direction::East);

                if !start_east && with_east {
                    start_east = true;
                    start_east_shape = shape;
                    continue;
                }
                if start_east && with_west {
                    if start_east_shape == 'F' && shape == 'J'
                        || start_east_shape == 'L' && shape == '7'
                    {
                        // simliar like '|'
                        count += 1;
                    }
                    start_east = false;
                }
            } else {
                if count % 2 == 1 {
                    result += 1;
                }
            }
        }
    }
    result
}
