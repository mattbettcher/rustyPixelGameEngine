use crate::{Pixel};
use super::vec3d::Vec3d;
use super::vec4d::Vec4d;

pub struct Triangle {
    pub p: [Vec4d; 3],
    pub t: [Vec3d; 3],
    pub col: Pixel,
}