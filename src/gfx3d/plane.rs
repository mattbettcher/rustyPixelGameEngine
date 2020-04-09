
use super::vec4d::Vec4d;

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub position: Vec4d,
    pub normal: Vec4d,
}

impl Plane {
    pub fn line_intersect_plane(&self, start: Vec4d, end: Vec4d) -> (Vec4d, f32) {
        let n = self.normal.norm();
        let d = -n.dot(&self.position);
        let ad = start.dot(&n);
        let bd = end.dot(&n);
        let t = (-d - ad) / (bd - ad);
        let start_to_end = end - start;
        let to_intersect = start_to_end * t;
        (start + to_intersect, t)
    }
}