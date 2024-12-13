use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util::io;

struct Sides {
    lr_sides: Vec<(i32, Vec<i32>)>,
    lri_sides: Vec<(i32, Vec<i32>)>,
    ud_sides: Vec<(i32, Vec<i32>)>,
    udi_sides: Vec<(i32, Vec<i32>)>,
}

impl Sides {
    fn new() -> Sides {
        Sides {
            lr_sides: Vec::new(),
            lri_sides: Vec::new(),
            ud_sides: Vec::new(),
            udi_sides: Vec::new(),
        }
    }

    // y is relevant, connect neighour to =+X
    fn connect_or_add_lr(&mut self, x: i32, y: i32) {
        let mut found = false;
        for (key, side_points) in self.lr_sides.iter_mut() {
            if *key == y {
                if side_points.contains(&(x + 1)) || side_points.contains(&(x - 1)) {
                    found = true;
                    side_points.push(x);
                    break;
                }
            }
        }
        if !found {
            self.lr_sides.push((y, vec![x]));
        }
    }

    fn connect_or_add_lri(&mut self, x: i32, y: i32) {
        let mut found = false;
        for (key, side_points) in self.lri_sides.iter_mut() {
            if *key == y {
                if side_points.contains(&(x + 1)) || side_points.contains(&(x - 1)) {
                    found = true;
                    side_points.push(x);
                    break;
                }
            }
        }
        if !found {
            self.lri_sides.push((y, vec![x]));
        }
    }

    fn connect_or_add_ud(&mut self, x: i32, y: i32) {
        let mut found = false;
        for (key, side_points) in self.ud_sides.iter_mut() {
            if *key == x {
                if side_points.contains(&(y + 1)) || side_points.contains(&(y - 1)) {
                    found = true;
                    side_points.push(y);
                    break;
                }
            }
        }
        if !found {
            self.ud_sides.push((x, vec![y]));
        }
    }

    fn connect_or_add_udi(&mut self, x: i32, y: i32) {
        let mut found = false;
        for (key, side_points) in self.udi_sides.iter_mut() {
            if *key == x {
                if side_points.contains(&(y + 1)) || side_points.contains(&(y - 1)) {
                    found = true;
                    side_points.push(y);
                    break;
                }
            }
        }
        if !found {
            self.udi_sides.push((x, vec![y]));
        }
    }

    fn count(&self) -> i32 {
        self.lr_sides.len() as i32
            + self.lri_sides.len() as i32
            + self.ud_sides.len() as i32
            + self.udi_sides.len() as i32
    }
}

#[derive(Debug)]
struct Region {
    id: char,
    positions: Vec<(i32, i32)>,
}

impl Region {
    fn get_area(&self) -> i32 {
        self.positions.len() as i32
    }

    fn calc_perimeter(&self) -> (i32, i32) {
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

        let mut sides: Sides = Sides::new();

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
                        sides.connect_or_add_ud(x, y);
                    }
                } else {
                    // go out of region
                    if in_region {
                        in_region = false;
                        perimeter += 1;
                        sides.connect_or_add_udi(x, y);
                    }
                }
            }
        }

        // check columns from miny to maxy (over all x)
        for x in minx..=maxx {
            let mut in_region = false;
            // +1 to get the right border (from region to nothing)
            for y in miny..=maxy + 1 {
                if positions.contains(&(x, y)) {
                    // go in region
                    if !in_region {
                        in_region = true;
                        perimeter += 1;
                        sides.connect_or_add_lr(x, y);
                    }
                } else {
                    // go out of region
                    if in_region {
                        in_region = false;
                        perimeter += 1;
                        sides.connect_or_add_lri(x, y);
                    }
                }
            }
        }

        (perimeter, sides.count())
    }

    fn calc_price_perimeter(&self) -> i32 {
        let (perimeter, sides) = self.calc_perimeter();
        self.get_area() * perimeter
    }

    fn calc_price_sides(&self) -> i32 {
        let (perimeter, sides) = self.calc_perimeter();
        self.get_area() * sides
    }
}

#[derive(Debug)]
struct Plant {
    id: char,
    pub positions: Vec<(i32, i32)>,
}

impl Plant {
    pub fn split_into_regions(&mut self) -> Vec<Region> {
        let areas = split_into_areas(&self.positions)
            .iter()
            .map(|area| Region {
                id: self.id,
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
                    id: c,
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

    part2(&regions);
}

fn part1(regions: &[Region]) {
    let mut total_price = 0;
    for region in regions.iter() {
        // println!("{:?} / {:?}", region.id, region.calc_price());

        total_price += region.calc_price_perimeter();
    }

    println!("Day12 part 1: {:?}", total_price);
}

fn part2(regions: &[Region]) {
    let mut total_price = 0;
    for region in regions.iter() {
        // println!("{:?} / {:?}", region.id, region.calc_price());

        total_price += region.calc_price_sides();
    }

    println!("Day12 part 2: {:?}", total_price);
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
