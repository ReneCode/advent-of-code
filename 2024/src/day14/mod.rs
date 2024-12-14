use itertools::Itertools;

use crate::util::io;

type Number = i32;

const SPACE_WIDE: Number = 101;
const SPACE_TALL: Number = 103;

#[derive(Clone, Debug)]
struct Vector {
    x: Number,
    y: Number,
}

impl Vector {
    // "3,-3" => Vector { x: 3, y: -3 }
    fn from(line: &str) -> Vector {
        let tok = line
            .split(",")
            .map(|x| x.trim())
            .map(|x| x.parse::<Number>().unwrap())
            .collect_vec();

        Vector {
            x: tok[0],
            y: tok[1],
        }
    }
}

#[derive(Debug)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl Robot {
    fn from(line: &str) -> Robot {
        let vectors = line
            .split(" ")
            .map(|x| x.trim())
            .map(|x| x.split("=").collect_vec())
            .map(|t| Vector::from(t[1]))
            .collect_vec();

        Robot {
            position: vectors[0].clone(),
            velocity: vectors[1].clone(),
        }
    }

    fn tick(&mut self, wide: Number, tall: Number) {
        self.position = Vector {
            x: (wide + self.position.x + self.velocity.x) % wide,
            y: (tall + self.position.y + self.velocity.y) % tall,
        }
    }
}

pub fn day14() {
    let lines = io::read_lines("./src/day14/14.data").unwrap();

    let mut robots = lines.iter().map(|line| Robot::from(line)).collect_vec();
    part1(&mut robots);
}

fn part1(robots: &mut [Robot]) {
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            // print!("Robot:{:?} => {:?}  ", robot.velocity, robot.position);
            robot.tick(SPACE_WIDE, SPACE_TALL);
            // println!("{:?}", robot.position);
        }
    }

    let mut quadrants = vec![0; 4];

    let half_wide = SPACE_WIDE / 2;
    let half_tall = SPACE_TALL / 2;

    for robot in robots.iter() {
        let quad_index = if robot.position.y < half_tall {
            if robot.position.x < half_wide {
                0
            } else if robot.position.x > half_wide {
                1
            } else {
                -1
            }
        } else if robot.position.y > half_tall {
            if robot.position.x < half_wide {
                2
            } else if robot.position.x > half_wide {
                3
            } else {
                -1
            }
        } else {
            -1
        };
        if quad_index >= 0 {
            quadrants[quad_index as usize] += 1;
        }
    }

    let result = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];

    println!("Day14 part 1: {:?} / {:?}", quadrants, result);
}
