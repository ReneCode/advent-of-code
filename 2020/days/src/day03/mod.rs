// day03

use crate::util::{
    io,
    point2d::{self, Point2d},
};

struct Area {
    lines: Vec<String>,
    size: Point2d,
}

impl Area {
    fn new(lines: Vec<String>) -> Self {
        let size = point2d::Point2d::new(lines[0].len() as i32, lines.len() as i32);
        Area {
            lines: lines,
            size: size,
        }
    }

    fn value(&self, point: &point2d::Point2d) -> char {
        let x_idx = (point.x % self.size.x) as usize;
        let y_idx = (point.y % self.size.y) as usize;
        self.lines.get(y_idx).unwrap().chars().nth(x_idx).unwrap()
    }
}

pub fn day03() {
    println!("hello day03");

    let lines = io::read_lines("./03.data").unwrap();
    let area = Area::new(lines);

    let pt_move = point2d::Point2d::new(3, 1);
    let mut point = pt_move.clone();
    let mut count_tree = 0;
    while point.y < area.size.y {
        match area.value(&point) {
            '.' => {}
            '#' => {
                count_tree += 1;
            }
            _ => {
                panic!("bad area value value")
            }
        }
        println!("result size {:?} {count_tree}", point);
        point = point.add(&pt_move);
    }
}
