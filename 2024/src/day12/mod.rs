use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::io;

#[derive(Debug)]
struct Region {
    positions: Vec<(i32, i32)>,
}

impl Region {
    fn get_area(&self) -> i32 {
        self.positions.len() as i32
    }

    fn calc_perimeter(&self) -> i32 {
        let (minx, miny, maxx, maxy) = (
            self.positions.iter().map(|p| p.0).min().unwrap(),
            self.positions.iter().map(|p| p.1).min().unwrap(),
            self.positions.iter().map(|p| p.0).max().unwrap(),
            self.positions.iter().map(|p| p.1).max().unwrap(),
        );

        let mut positions: HashSet<(i32, i32)> = HashSet::new();
        for pos in self.positions.iter() {
            positions.insert(*pos);
        }

        // check rows from minx to maxx (over all y)

        let mut perimeter = 0;
        for y in miny..=maxy {
            let mut in_region = false;
            // +1 to get the right border (from region to nothing)
            for x in minx..=maxx + 1 {
                if positions.contains(&(x, y)) {
                    // go in region
                    if !in_region {
                        in_region = true;
                        perimeter += 1;
                    }
                } else {
                    // go out of region
                    if in_region {
                        in_region = false;
                        perimeter += 1;
                    }
                }
            }
        }

        for x in minx..=maxx {
            let mut in_region = false;
            // +1 to get the right border (from region to nothing)
            for y in miny..=maxy + 1 {
                if positions.contains(&(x, y)) {
                    // go in region
                    if !in_region {
                        in_region = true;
                        perimeter += 1;
                    }
                } else {
                    // go out of region
                    if in_region {
                        in_region = false;
                        perimeter += 1;
                    }
                }
            }
        }

        perimeter
    }

    fn calc_price(&self) -> i32 {
        self.get_area() * self.calc_perimeter()
    }
}

#[derive(Debug)]
struct Plant {
    pub positions: Vec<(i32, i32)>,
}

impl Plant {
    pub fn split_into_regions(&mut self) -> Vec<Region> {
        let areas = split_into_areas(&self.positions)
            .iter()
            .map(|area| Region {
                positions: area.clone(),
            })
            .collect_vec();
        areas
    }
}

pub fn day12() {
    let lines = io::read_lines("./src/day12/12.data").unwrap();

    let mut plants: HashMap<char, Plant> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            plants
                .entry(c)
                .and_modify(|plant| plant.positions.extend(vec![(x as i32, y as i32)]))
                .or_insert(Plant {
                    positions: vec![(x as i32, y as i32)],
                });
        }
    }

    let mut regions: Vec<Region> = Vec::new();
    for (_c, plant) in plants.iter_mut() {
        let plant_regions = plant.split_into_regions();
        regions.extend(plant_regions);
    }

    part1(&regions);
}

fn part1(regions: &[Region]) {
    let mut total_price = 0;
    for region in regions.iter() {
        // println!("{:?} / {:?}", region.id, region.calc_price());

        total_price += region.calc_price();
    }

    println!("Day12 part 1: {:?}", total_price);
}

// a list of positions, split into areas where all positions are connected +-X or +-Y (neighbours)
fn split_into_areas(positions: &[(i32, i32)]) -> Vec<Vec<(i32, i32)>> {
    let mut areas: Vec<Vec<(i32, i32)>> = vec![];

    let mut temp: HashSet<(i32, i32)> = HashSet::new();
    positions.iter().for_each(|p| {
        temp.insert(*p);
    });

    while !temp.is_empty() {
        let mut current_area: Vec<(i32, i32)> = vec![];
        let mut check_positions: Vec<(i32, i32)> = vec![];

        let one = temp.iter().cloned().collect_vec();
        let first_pos = one.first().unwrap();

        check_positions.push(*first_pos);
        while let Some((x, y)) = check_positions.pop() {
            if temp.contains(&(x, y)) {
                temp.remove(&(x, y));
                current_area.push((x, y));
                check_positions.push((x + 1, y));
                check_positions.push((x - 1, y));
                check_positions.push((x, y + 1));
                check_positions.push((x, y - 1));
            }
        }
        areas.push(current_area);
    }

    areas
}
