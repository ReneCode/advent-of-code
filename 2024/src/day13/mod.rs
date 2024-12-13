use crate::util::io;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Machine {
    move_a: Point,
    move_b: Point,
    prize: Point,
}

impl Machine {
    fn from(lines: &[&str]) -> Self {
        fn parse(line: &str) -> Vec<&str> {
            line.split(":")
                .last()
                .unwrap()
                .split(",")
                .map(|s| s.trim())
                .map(|s| &s[1..]) // remove X / Y
                .collect_vec()
        }
        let tok = parse(lines[0])
            .iter()
            .map(|s| s.parse().unwrap())
            .collect_vec();
        let move_a = Point {
            x: tok[0],
            y: tok[1],
        };

        let tok = parse(lines[1])
            .iter()
            .map(|s| s.parse().unwrap())
            .collect_vec();
        let move_b = Point {
            x: tok[0],
            y: tok[1],
        };

        let tok = parse(lines[2])
            .iter()
            .map(|s| &s[1..]) // remote =
            .map(|s| s.parse().unwrap())
            .collect_vec();
        let prize = Point {
            x: tok[0],
            y: tok[1],
        };

        Machine {
            move_a,
            move_b,
            prize,
        }
    }

    fn get_button_count(&self) -> Option<(i32, i32)> {
        // production optimization
        // just simple algebra to solve the equation for a and
        let b = (self.prize.y * self.move_a.x - self.move_a.y * self.prize.x)
            / (self.move_a.x * self.move_b.y - self.move_a.y * self.move_b.x);

        let a = (self.prize.x - b * self.move_b.x) / self.move_a.x;

        let target = Point {
            x: a * self.move_a.x + b * self.move_b.x,
            y: a * self.move_a.y + b * self.move_b.y,
        };

        if self.prize == target {
            Some((a, b))
        } else {
            None
        }
    }

    fn get_cost_to_solve(&self) -> i32 {
        if let Some((a, b)) = self.get_button_count() {
            a * 3 + b
        } else {
            0
        }
    }
}

pub fn day13() {
    let lines = io::read_lines("./src/day13/13.data").unwrap();

    let all_lines = lines.join("\n");
    let machines = all_lines
        .split("\n\n")
        .map(|ls| ls.split('\n').collect_vec())
        .map(|l3| Machine::from(&l3))
        .collect_vec();

    let mut sum_cost = 0;
    for machine in machines {
        let cost = machine.get_cost_to_solve();
        sum_cost += cost;
    }

    println!("day13 part1: {:?}", sum_cost);
}
