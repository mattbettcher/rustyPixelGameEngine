use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3d { x, y, z, w: 1.0 }
    }

    pub fn length(&self) -> f32 {
        (self.dot(self)).sqrt()
    }

    pub fn norm(&self) -> Self {
        let l = 1.0 / self.length();
        Vec3d { x: self.x * l, y: self.y * l, z: self.z * l, w: 1.0 }
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Vec3d {
        Vec3d {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
            w: 1.0
        }
    }
}

impl Add for Vec3d {
    type Output = Vec3d;

    fn add(self, rhs: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: 1.0,
        }
    }
}

impl Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, rhs: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: 1.0,
        }
    }
}

impl Mul<f32> for Vec3d {
    type Output = Vec3d;

    fn mul(self, rhs: f32) -> Vec3d {
        Vec3d {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: 1.0,
        }
    }
}

impl Div<f32> for Vec3d {
    type Output = Vec3d;

    fn div(self, rhs: f32) -> Vec3d {
        Vec3d {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: 1.0,
        }
    }
}

impl AddAssign for Vec3d {
    fn add_assign(&mut self, rhs: Vec3d) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl SubAssign for Vec3d {
    fn sub_assign(&mut self, rhs: Vec3d) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl MulAssign<f32> for Vec3d {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl DivAssign<f32> for Vec3d {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}
