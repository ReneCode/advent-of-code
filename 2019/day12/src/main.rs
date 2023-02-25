extern crate util;

use util::io;
use util::point_3d::Point3d;

#[derive(Debug)]
struct Moon {
    position: Point3d,
    velocity: Point3d,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        let position = Point3d::new(x, y, z);
        let velocity: Point3d = Point3d::new(0, 0, 0);
        Moon { position, velocity }
    }

    fn inc_velocity(&mut self, gravity: &Point3d) {
        self.velocity = self.velocity.add(gravity);
    }

    fn dec_velocity(&mut self, gravity: &Point3d) {
        self.velocity = self.velocity.sub(gravity);
    }

    fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn get_total_energy(&self) -> i32 {
        let pot = self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let kin = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
        pot * kin
    }

    fn print(&self) {
        println!(
            "pos=<x:{}, y:{}, z:{}>, vel=<x:{}, y:{}, z:{}> total: {}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z,
            self.get_total_energy()
        );
    }
}

fn main() {
    println!("Hello, day12!");
    if let Some(mut moons) = get_data("12.data") {
        part_1(&mut moons);
    }
}

fn get_data(filename: &str) -> Option<Vec<Moon>> {
    if let Some(lines) = io::get_lines(filename) {
        let mut moons: Vec<Moon> = Vec::new();
        for line in lines {
            let mut a = line.strip_prefix("<").unwrap();
            a = a.strip_suffix(">").unwrap();
            let coords: Vec<i32> = a
                .split(",")
                .map(|t| {
                    let tok: Vec<&str> = t.split("=").collect();
                    let nr = tok[1].parse::<i32>().unwrap();
                    nr
                })
                .collect();
            let moon = Moon::new(coords[0], coords[1], coords[2]);
            moons.push(moon);
        }
        Some(moons)
    } else {
        None
    }
}

fn part_1(moons: &mut Vec<Moon>) {
    // print_moons(moons, 0);

    let steps = 1000;
    for step in 1..=steps {
        let combinations = get_combinations_idx(moons.len());
        for combination in combinations {
            let m1 = moons.get(combination.0).unwrap();
            let m2 = moons.get(combination.1).unwrap();
            let gravity = calc_gravity(&m1, &m2);
            moons.get_mut(combination.0).unwrap().inc_velocity(&gravity);
            moons.get_mut(combination.1).unwrap().dec_velocity(&gravity);
        }
        for moon in moons.iter_mut() {
            moon.update_position();
        }

        // print_moons(moons, step);
    }
    let sum_total_entergy: i32 = moons.iter().map(|m| m.get_total_energy()).sum();
    println!(
        "part-1: sum total entergy after {} steps: {}",
        steps, sum_total_entergy,
    );
}

fn print_moons(moons: &Vec<Moon>, step: usize) {
    println!("After {} steps:", step);
    for moon in moons {
        moon.print()
    }
    println!("");
}

fn calc_gravity(m1: &Moon, m2: &Moon) -> Point3d {
    let mut gravity = Point3d::new(0, 0, 0);
    gravity.x += get_gravity(m1.position.x, m2.position.x);
    gravity.y += get_gravity(m1.position.y, m2.position.y);
    gravity.z += get_gravity(m1.position.z, m2.position.z);
    gravity
}

fn get_gravity(g1: i32, g2: i32) -> i32 {
    if g1 > g2 {
        return -1;
    }
    if g1 < g2 {
        return 1;
    }
    0
}

fn get_combinations_idx(len: usize) -> Vec<(usize, usize)> {
    let mut combinations: Vec<(usize, usize)> = Vec::new();
    for i in 0..len {
        for j in i + 1..len {
            combinations.push((i, j))
        }
    }
    combinations
}
