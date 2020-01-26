use crate::{Pixel};

pub mod vec3d;
pub mod mat4x4;

use self::vec3d::Vec3d;

pub struct Vec2d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Triangle {
    pub p: [Vec3d; 3],
    pub t: [Vec2d; 3],
    pub col: Pixel,
}



pub struct Mesh {
    pub tris: Vec<Triangle>
}
