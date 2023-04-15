use std::fmt::Display;

#[derive(Hash, Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
// impl Clone for Point {
//     fn clone(&self) -> Self {
//         Point {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Eq for Point {}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct BoundingBox {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,

    started: bool,
}

impl BoundingBox {
    pub fn new() -> Self {
        BoundingBox {
            x_min: 0,
            x_max: 0,
            y_min: 0,
            y_max: 0,
            started: false,
        }
    }

    pub fn add(&mut self, point: &Point) {
        if !self.started {
            self.x_min = point.x;
            self.x_max = point.x;
            self.x_min = point.y;
            self.y_max = point.y;
            self.started = true;
        } else {
            self.x_min = self.x_min.min(point.x);
            self.x_max = self.x_max.max(point.x);
            self.y_min = self.y_min.min(point.y);
            self.y_max = self.y_max.max(point.y);
        }
    }
}
