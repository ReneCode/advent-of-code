// Point2d

#[derive(Debug, Clone)]
pub struct Point2d {
    pub x: i32,
    pub y: i32,
}

impl Point2d {
    pub fn new(x: i32, y: i32) -> Self {
        Point2d { x: x, y: y }
    }

    pub fn add(&self, other: &Point2d) -> Self {
        Point2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
