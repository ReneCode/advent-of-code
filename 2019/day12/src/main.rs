extern crate util;

use util::io;
use util::point_3d::Point3d;

use num_integer;

#[derive(Debug, PartialEq, Eq, Clone)]
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

enum CompareOption {
    PosX,
    PosY,
    PosZ,
    VelX,
    VelY,
    VelZ,
}

fn main() {
    println!("Hello, day12!");
    let filename = "12.data";
    if let Some(mut moons) = get_data(filename) {
        part_1(&mut moons);
    }
    if let Some(mut moons) = get_data(filename) {
        part_2(&mut moons);
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

    let steps = 10;
    for step in 1..=steps {
        calc_one_step(moons);
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

fn calc_one_step(moons: &mut Vec<Moon>) {
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
}

fn part_2(moons: &Vec<Moon>) {
    let start_moons: Vec<Moon> = moons.iter().map(|m| m.clone()).collect();

    // calc periodic length for each coord for position and velocity
    let mut periodic_lengths: Vec<usize> = Vec::new();
    for option in vec![
        CompareOption::PosX,
        CompareOption::PosY,
        CompareOption::PosZ,
        CompareOption::VelX,
        CompareOption::VelY,
        CompareOption::VelZ,
    ] {
        let mut steps = 0;
        let mut moons: Vec<Moon> = start_moons.iter().map(|m| m.clone()).collect();
        // last and first positions of a periode are same. because of velocity = 0,0,0
        // last-positions if one step too less for a complete periode.
        match option {
            CompareOption::PosX => steps += 1,
            CompareOption::PosY => steps += 1,
            CompareOption::PosZ => steps += 1,
            _ => {}
        }
        loop {
            calc_one_step(&mut moons);

            steps += 1;

            if is_equal_with_option(&start_moons, &moons, &option) {
                // println!("periode found {}", steps);
                periodic_lengths.push(steps);
                break;
            }
        }
    }

    // calc the least common multiplier of all periode lengths
    let mut lcm = 1;
    for len in periodic_lengths {
        lcm = num_integer::lcm(lcm, len);
    }

    // let sum_total_entergy: i32 = moons.iter().map(|m| m.get_total_energy()).sum();
    println!("part-2: result: {}", lcm);
}

fn is_equal_with_option(ma: &Vec<Moon>, mb: &Vec<Moon>, cmp_option: &CompareOption) -> bool {
    for i in 0..ma.len() {
        let va = &ma[i];
        let vb = &mb[i];
        let val_equal = match cmp_option {
            CompareOption::PosX => va.position.x == vb.position.x,
            CompareOption::PosY => va.position.y == vb.position.y,
            CompareOption::PosZ => va.position.z == vb.position.z,
            CompareOption::VelX => va.velocity.x == vb.velocity.x,
            CompareOption::VelY => va.velocity.y == vb.velocity.y,
            CompareOption::VelZ => va.velocity.z == vb.velocity.z,
        };
        if !val_equal {
            return false;
        }
    }

    true
}

fn is_equal(ma: &Vec<Moon>, mb: &Vec<Moon>) -> bool {
    let mut equal = true;
    for i in 0..ma.len() {
        let va = &ma[i];
        let vb = &mb[i];
        if va != vb {
            equal = false;
            break;
        }
    }

    equal
}
