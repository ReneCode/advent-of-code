// day10

use core::num;
use std::collections::HashMap;

use itertools::Itertools;

use crate::util::{io, math, parse};

#[derive(Debug, PartialEq, Copy, Clone)]
struct Tile {
    x: i32,
    y: i32,
}

impl Tile {
    fn new(x: i32, y: i32) -> Self {
        Tile { x: x, y: y }
    }
}

struct Area {
    lines: Vec<String>,
    x_len: i32,
    y_len: i32,
}

impl Area {
    fn new(lines: Vec<String>) -> Self {
        Area {
            y_len: lines.len() as i32,
            x_len: lines[0].len() as i32,
            lines: lines,
        }
    }

    fn find_tile(&self, c: char) -> Option<Tile> {
        for (row, line) in self.lines.iter().enumerate() {
            if let Some(col) = line.find(c) {
                return Some(Tile::new(col as i32, row as i32));
            }
        }
        None
    }

    fn get_loop(&self, start_tile: &Tile) -> Vec<Tile> {
        let mut result: Vec<Tile> = Vec::new();
        result.push(start_tile.clone());
        let mut tile = start_tile.clone();
        loop {
            let next_tiles = self.get_next_tiles(&tile);
            if let Some(next_tile) = next_tiles
                .iter()
                .find(|t| !result.contains(t) && self.get_shape(t) != '.')
            {
                result.push(next_tile.clone());
                tile = next_tile.clone();
            } else {
                break;
            }
        }

        result
    }

    fn get_shape(&self, tile: &Tile) -> char {
        self.lines[tile.y as usize]
            .chars()
            .nth(tile.x as usize)
            .unwrap()
    }

    fn get_next_tiles(&self, tile: &Tile) -> Vec<Tile> {
        let mut result: Vec<Tile> = Vec::new();
        let shape = self.get_shape(&tile);
        let xy_deltas: Vec<(i32, i32)> = match shape {
            'S' => {
                vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            }
            '|' => {
                vec![(0, -1), (0, 1)]
            }
            '-' => {
                vec![(-1, 0), (1, 0)]
            }
            'L' => {
                vec![(1, 0), (0, -1)]
            }
            'J' => {
                vec![(-1, 0), (0, -1)]
            }
            '7' => {
                vec![(-1, 0), (0, 1)]
            }
            'F' => {
                vec![(1, 0), (0, 1)]
            }
            '.' => {
                vec![]
            }
            _ => panic!("bad shape"),
        };
        for (dx, dy) in xy_deltas {
            if (0..self.x_len).contains(&(tile.x + dx)) && (0..self.y_len).contains(&(tile.y + dy))
            {
                result.push(Tile::new(tile.x + dx, tile.y + dy))
            }
        }
        result
    }
}

pub fn day10() {
    println!("hello day10");

    let lines = io::read_lines("./src/day10/10-example.data").unwrap();

    let area = Area::new(lines);
    let start_tile = area.find_tile('S').unwrap();
    let tile_loop: Vec<Tile> = area.get_loop(&start_tile);

    println!("{:?}", tile_loop);
    let result_a = tile_loop.len() / 2;
    println!("Result A: {result_a}");
}
