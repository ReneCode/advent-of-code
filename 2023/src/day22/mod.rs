// day22

use std::{
    collections::{HashMap, HashSet},
    result,
    str::FromStr,
};

use itertools::Itertools;

use crate::util::io;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(Debug)]
struct ParsePositionError;
impl FromStr for Position {
    type Err = ParsePositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z): (i32, i32, i32) = s
            .split(',')
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple::<(i32, i32, i32)>()
            .unwrap();

        Ok(Position { x, y, z })
    }
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }
}

#[derive(Debug, Clone)]
struct Brick {
    idx: usize,
    positions: Vec<Position>,
}
impl Brick {
    fn new(idx: usize, positions: Vec<Position>) -> Self {
        Brick { idx, positions }
    }

    fn contains_pos(&self, check_pos: &Position) -> bool {
        for pos in self.positions.iter() {
            if pos == check_pos {
                return true;
            }
        }
        return false;
    }

    fn fall_down(&self) -> Brick {
        let positions = self
            .positions
            .iter()
            .map(|p| Position::new(p.x, p.y, p.z - 1))
            .collect_vec();
        Brick::new(self.idx, positions)
    }

    fn lowest_z(&self) -> i32 {
        let min_z = self.positions.iter().map(|p| p.z).min().unwrap();
        min_z
    }

    fn intersect(&self, other: &Brick) -> bool {
        for pos in self.positions.iter() {
            for other_pos in other.positions.iter() {
                if pos == other_pos {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug)]
struct ParseBrickError;
impl FromStr for Brick {
    type Err = ParseBrickError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok = s.split('~').collect_vec();
        let p1 = Position::from_str(tok[0]).unwrap();
        let p2 = Position::from_str(tok[1]).unwrap();
        let mut positions: Vec<Position> = Vec::new();
        if p1.y == p2.y && p1.z == p2.z {
            for x in norm_range(p1.x..=p2.x) {
                positions.push(Position::new(x, p1.y, p2.z));
            }
        } else if p1.x == p2.x && p1.z == p2.z {
            for y in norm_range(p1.y..=p2.y) {
                positions.push(Position::new(p1.x, y, p2.z));
            }
        } else if p1.y == p2.y && p1.x == p2.x {
            for z in norm_range(p1.z..=p2.z) {
                positions.push(Position::new(p1.x, p1.y, z));
            }
        } else {
            panic!("bad brick")
        }

        Ok(Brick { idx: 0, positions })
    }
}

pub fn day22() {
    println!("hello day22");

    let lines = io::read_lines("./src/day22/22.data").unwrap();
    let mut bricks: Vec<Brick> = Vec::new();
    for (idx, line) in lines.iter().enumerate() {
        let mut brick = Brick::from_str(&line).unwrap();
        brick.idx = idx;
        bricks.push(brick);
    }

    // println!("{:?}", bricks)

    let result_a = part_a(&bricks);
    println!("Result_A {result_a}")
}

fn norm_range(r: std::ops::RangeInclusive<i32>) -> std::ops::RangeInclusive<i32> {
    if r.start() <= r.end() {
        return r;
    } else {
        return *r.end()..=*r.start();
    }
}

fn part_a(all_bricks: &[Brick]) -> usize {
    let bricks = fall_down(all_bricks);

    let mut can_disintegrate: HashSet<usize> = HashSet::new();
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();
    for brick in bricks.iter() {
        let support_idx = support(&bricks, brick);
        if support_idx.len() == 0 {
            can_disintegrate.insert(brick.idx);
        } else {
            for idx in support_idx.iter() {
                if let Some(lower_brick_idx) = supported_by.get_mut(idx) {
                    lower_brick_idx.insert(brick.idx);
                } else {
                    let mut me: HashSet<usize> = HashSet::new();
                    me.insert(brick.idx);
                    supported_by.insert(*idx, me);
                }
            }
        }
    }

    for (idx, supporter) in supported_by.iter() {
        if supporter.len() > 1 {
            for idx in supporter.iter() {
                can_disintegrate.insert(*idx);
            }
        }
    }

    println!("{:?}", can_disintegrate);
    can_disintegrate.len()
}

fn fall_down(bricks: &[Brick]) -> Vec<Brick> {
    let mut result: Vec<Brick> = Vec::new();
    for brick in bricks {
        let mut ok_brick = brick.clone();
        let mut down_brick = brick.fall_down();
        while fits_into(&result, &down_brick) {
            ok_brick = down_brick.clone();
            down_brick = down_brick.fall_down();
        }
        result.push(ok_brick)
    }
    result
}

fn fits_into(bricks: &Vec<Brick>, brick: &Brick) -> bool {
    if brick.lowest_z() < 0 {
        return false;
    }
    for other in bricks.iter() {
        if brick.intersect(other) {
            return false;
        }
    }
    true
}

fn support(bricks: &[Brick], brick: &Brick) -> HashSet<usize> {
    let mut result: HashSet<usize> = HashSet::new();

    for upper_brick in bricks.iter().filter(|b| b.idx > brick.idx) {
        for pos in brick.positions.iter() {
            let check_pos = Position::new(pos.x, pos.y, pos.z + 1);
            if upper_brick.contains_pos(&check_pos) {
                result.insert(upper_brick.idx);
                break;
            }
        }
    }
    result
}
