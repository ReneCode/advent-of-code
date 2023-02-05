extern crate util;

const CENTER_OF_MASS: &str = "COM";

struct Orbit {
    center: String,
    outside: String,
    count_orbit: i32,
}
impl Orbit {
    fn new(center: &str, outside: &str) -> Orbit {
        Orbit {
            center: String::from(center),
            outside: String::from(outside),
            count_orbit: 0,
        }
    }
}

struct Universe {
    orbits: Vec<Orbit>,
}

impl Universe {
    fn get_center(&self, outside: &str) -> Option<String> {
        if let Some(found) = self.orbits.iter().find(|o| o.outside == outside) {
            Some(found.center.clone())
        } else {
            None
        }
    }

    fn count_center(&self, outside: &str) -> i32 {
        let mut total = 0;
        let mut current_outside = outside;
        while current_outside != CENTER_OF_MASS {
            if let Some(found) = self.orbits.iter().find(|o| o.outside == current_outside) {
                total += 1;
                current_outside = found.center.as_str();
            }
            // if let Some(found) = self.orbits.iter().find(|o| o.outside) == current_outside) {
            //     total += 1;
            //     current_outside = found.outside;
            // }
        }
        total
    }
}

fn main() {
    println!("Hello, day06!");
    if let Some(orbits) = get_data("./06.data") {
        part_1(&orbits);
    }
}

fn get_data(filename: &str) -> Option<Universe> {
    if let Some(input) = util::io::get_lines(filename) {
        let orbits: Vec<Orbit> = input
            .iter()
            .map(|line| {
                let tok: Vec<&str> = line.split(")").collect();
                let orbit = Orbit::new(tok[0], tok[1]);
                orbit
            })
            .collect();
        Some(Universe { orbits: orbits })
    } else {
        None
    }
}

fn part_1(universe: &Universe) {
    let mut total = 0;
    for orbit in universe.orbits.iter() {
        let count = universe.count_center(&orbit.outside);
        total += count;
    }

    println!("part1 total {}", total);
}
