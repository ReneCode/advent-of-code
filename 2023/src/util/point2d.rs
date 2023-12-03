use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Point2d {
    pub x: usize,
    pub y: usize,
}

impl Point2d {
    pub fn new(x: usize, y: usize) -> Self {
        Point2d { x: x, y: y }
    }
}
pub fn get_neighbours(pt: &Point2d, x_max: usize, y_max: usize) -> HashSet<Point2d> {
    let mut result: HashSet<Point2d> = HashSet::new();

    for x in (pt.x as i32 - 1).max(0)..(pt.x + 2).min(x_max) as i32 {
        for y in (pt.y as i32 - 1).max(0)..(pt.y + 2).min(y_max) as i32 {
            if x as usize != pt.x || y as usize != pt.y {
                result.insert(Point2d::new(x as usize, y as usize));
            }
        }
    }
    result
}
