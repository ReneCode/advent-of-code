use std::{collections::HashMap, os::unix::process::parent_id};

use itertools::Itertools;

use crate::util::io;

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    fn direct_neighbours(&self) -> Vec<Position> {
        vec![
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
        ]
    }
}

pub fn day20() {
    let lines = io::read_lines("./src/day20/20.data").unwrap();

    let mut grid: HashMap<Position, char> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(
                Position {
                    x: x as i32,
                    y: y as i32,
                },
                c,
            );
        }
    }

    part1(&grid);

    part2(&grid);
}

fn get_way_through_grid(grid: &HashMap<Position, char>) -> Vec<Position> {
    let start = grid.iter().find(|(_, &v)| v == START).unwrap().0;
    let end = grid.iter().find(|(_, &v)| v == END).unwrap().0;

    let mut way = vec![*start];
    let mut current_pos = *start;
    while current_pos != *end {
        let neighbours = current_pos
            .direct_neighbours()
            .iter()
            .filter(|pos| *pos != &current_pos)
            .filter(|pos| !way.contains(pos))
            .filter(|pos| {
                if let Some(&c) = grid.get(pos) {
                    c != WALL
                } else {
                    true
                }
            })
            .map(|pos| *pos)
            .collect_vec();

        assert_eq!(neighbours.len(), 1);

        let next_pos = neighbours.first().unwrap();
        way.push(next_pos.clone());
        current_pos = *next_pos;
    }
    way
}

fn part1(grid: &HashMap<Position, char>) {
    let way = get_way_through_grid(&grid);
    get_all_cheats_part1(&grid, &way);
}

fn part2(grid: &HashMap<Position, char>) {
    let way = get_way_through_grid(&grid);
    get_all_cheats_part2(&grid, &way);
}

fn get_all_cheats_part1(grid: &HashMap<Position, char>, way: &[Position]) {
    // println!("searching for cheats ....");

    let mut saved_way = HashMap::new();
    let wlen = way.len();

    // try to shorten the way from each position to a "later" position
    // by checking if there is a wall in between
    // and the points are 2 steps horizontally or vertically apart
    for (idx, wpos) in way.iter().enumerate() {
        for next_idx in idx + 1..wlen {
            if let Some(next_pos) = way.get(next_idx) {
                let dx = next_pos.x - wpos.x;
                let dy = next_pos.y - wpos.y;
                if dx.abs() == 2 && dy == 0 || dx == 0 && dy.abs() == 2 {
                    let middle_pos = Position::new(wpos.x + dx / 2, wpos.y + dy / 2);
                    if let Some(&c) = grid.get(&middle_pos) {
                        if c == WALL {
                            let saved_len = next_idx - idx - 2;
                            saved_way
                                .entry(saved_len)
                                .and_modify(|c| *c += 1)
                                .or_insert(1);
                            // println!("Cheating at {:?}", middle_pos);
                        }
                    }
                }
            }
        }
    }

    let mut all_count = 0;
    for (save, count) in saved_way.iter() {
        if *save >= 100 {
            all_count += *count;
        }
    }

    println!("Day20 part1:{:?}", all_count);
}

fn get_all_cheats_part2(grid: &HashMap<Position, char>, way: &[Position]) {
    // println!("searching for cheats ....");

    let mut saved_way = HashMap::new();
    let wlen = way.len();

    // try to shorten the way from each position to a "later" position
    for (idx, wpos) in way.iter().enumerate() {
        for final_idx in idx + 2..wlen {
            if let Some(final_pos) = way.get(final_idx) {
                let dx = final_pos.x - wpos.x;
                let dy = final_pos.y - wpos.y;

                // let check = Position::new(3, 7);
                // if final_pos == &check {
                //     println!("Check");
                // }

                let cheat_len = (dx.abs() + dy.abs()) as usize;
                if cheat_len <= 20 {
                    let saved_len = final_idx - idx - cheat_len;
                    saved_way
                        .entry(saved_len)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                    // println!("Cheating at {:?} / {:?}", wpos, final_pos);
                }
            }
        }
    }

    let mut all_count = 0;
    for (save, count) in saved_way.iter() {
        if *save >= 100 {
            all_count += *count;
        }
    }

    println!("Day20 part2: {:?}", all_count);
}
