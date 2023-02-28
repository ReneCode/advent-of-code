#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3d {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Point3d { x, y, z }
    }

    pub fn add(&self, other: &Self) -> Self {
        Point3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Point3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
