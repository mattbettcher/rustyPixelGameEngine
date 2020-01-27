use crate::{Pixel};

pub mod vec3d;
pub mod vec4d;
pub mod mat4x4;

use self::vec3d::Vec3d;
use self::vec4d::Vec4d;

pub struct Triangle {
    pub p: [Vec4d; 3],
    pub t: [Vec3d; 3],
    pub col: Pixel,
}

pub struct Mesh {
    pub tris: Vec<Triangle>
}
