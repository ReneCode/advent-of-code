use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::io;

pub fn day18() {
    let lines = io::read_lines("./src/day18/18.data").unwrap();

    let mut area: HashSet<(i32, i32)> = HashSet::new();

    let positions = lines
        .iter()
        .map(|line| {
            let tok = line
                .split(",")
                .map(|x| x.trim())
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();

            let key = (tok[0], tok[1]);
            key
            // area.insert(key);
        })
        .collect_vec();

    part1(&positions[0..1024]);

    part2(&positions);
}

fn part1(positions: &[(i32, i32)]) {
    if let Some(result) = get_way(positions) {
        println!("Day18 part 1: {:?}", result);
    }
}

fn part2(positions: &[(i32, i32)]) {
    for i in 1024..positions.len() {
        if let Some(result) = get_way(&positions[0..i]) {
            println!("Day18 part 2: {:?}", i);
        } else {
            println!("Day18 part 2: {:?} => {:?}", i, positions[i - 1]);
            break;
        }
    }
}

fn get_way(positions: &[(i32, i32)]) -> Option<i32> {
    let start = (0, 0);
    let mut end = (0, 0);

    let mut area: HashSet<(i32, i32)> = HashSet::new();
    for pos in positions {
        if pos.0 > end.0 {
            end.0 = pos.0;
        }
        if pos.1 > end.1 {
            end.1 = pos.1;
        }
        area.insert(*pos);
    }

    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();
    dist.insert(start, 0);

    let mut prio_queue: Vec<((i32, i32), i32)> = vec![(start, 0)];
    while !prio_queue.is_empty() {
        let mut min_dist = prio_queue.first().unwrap().1;
        let mut min_index = 0;
        for (i, x) in prio_queue.iter().enumerate() {
            if x.1 < min_dist {
                min_dist = x.1;
                min_index = i;
            }
        }
        let (min_point, min_dist) = prio_queue.remove(min_index);

        // let (min_point, min_dist) = prio_queue.remove(0);

        let (x, y) = min_point;
        let around_neigbours = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        let neigbours = around_neigbours
            .iter()
            .filter(|x| x.0 >= 0 && x.0 <= end.0 && x.1 >= 0 && x.1 <= end.1)
            .filter(|x| !area.contains(x))
            .collect_vec();

        for n in neigbours {
            let new_dist = min_dist + 1;
            if let Some(d) = dist.get_mut(&n) {
                if new_dist < *d {
                    *d = new_dist;
                    prio_queue.push((n.clone(), new_dist));
                }
            } else {
                dist.insert(n.clone(), new_dist);
                prio_queue.push((n.clone(), new_dist));
            }
        }
    }

    if let Some(shortest_way) = dist.get(&end) {
        Some(*shortest_way)
    } else {
        None
    }
}
