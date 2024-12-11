use std::collections::HashSet;

use crate::util::io;
use itertools::Itertools;

pub fn day10() {
    let lines = io::read_lines("./src/day10/10.data").unwrap();

    let grid = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => -1,
                    _ => c.to_digit(10).unwrap() as i32,
                })
                .collect_vec()
        })
        .collect_vec();

    part1_2(&grid);
}

fn part1_2(grid: &[Vec<i32>]) {
    let mut start_pos = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 0 {
                start_pos.push((x, y));
            }
        }
    }

    let mut result_p1 = 0;
    let mut result_p2 = 0;
    for (x, y) in &start_pos {
        let (count_p1, count_p2) = count_ways(grid, (*x as i32, *y as i32));
        result_p1 += count_p1;
        result_p2 += count_p2;
    }

    println!("Day 10 Part1: {}", result_p1);
    println!("Day 10 Part2: {}", result_p2);
}

fn count_ways(grid: &[Vec<i32>], start_pos: (i32, i32)) -> (i32, i32) {
    let x_max = grid[0].len() as i32;
    let y_max = grid.len() as i32;

    let mut reached_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut reached_count = 0;
    let mut ways = vec![vec![(start_pos.0, start_pos.1, 0)]];
    while let Some(way) = ways.pop() {
        // let way = ways.pop().unwrap();
        let (x, y, id) = way.last().unwrap();
        let mut neighbors = vec![];
        if *x > 0 {
            neighbors.push((x - 1, *y));
        }
        if *x < x_max - 1 {
            neighbors.push((x + 1, *y));
        }
        if *y > 0 {
            neighbors.push((*x, y - 1));
        }
        if *y < y_max - 1 {
            neighbors.push((*x, y + 1));
        }
        let next_id = id + 1;
        for (neigbour_x, neighbour_y) in neighbors {
            if grid[neighbour_y as usize][neigbour_x as usize] == next_id {
                if next_id == 9 {
                    reached_positions.insert((neigbour_x, neighbour_y));
                    reached_count += 1;
                } else {
                    let mut new_way = way.clone();
                    new_way.push((neigbour_x, neighbour_y, next_id));
                    ways.push(new_way);
                }
            }
        }
    }

    (reached_positions.len() as i32, reached_count)
}
