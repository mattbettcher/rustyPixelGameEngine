

use crate::{Pixel};
use super::vec3d::Vec3d;
use super::vec4d::Vec4d;
use super::plane::Plane;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p: [Vec4d; 3], 
    pub t: [Vec3d; 3],
    pub col: Pixel,
}

impl Triangle {

    pub fn new() -> Self {
        Triangle { p: [Vec4d::zero(); 3], t: [Vec3d::zero(); 3], col: Pixel::rgb(0, 0, 0) }
    }

    pub fn clip_against_plane(&self, plane: &Plane) -> Vec<Triangle> {
        let pn = plane.normal.norm();

        let mut inside_points: [Vec4d; 3] = [Vec4d::zero(); 3];
        let mut outside_points: [Vec4d; 3] = [Vec4d::zero(); 3];
        let mut inside_tex: [Vec3d; 3] = [Vec3d::zero(); 3];
        let mut outside_tex: [Vec3d; 3] = [Vec3d::zero(); 3];

        let mut inside_count = 0;
        let mut outside_count = 0;

        let dist = |p: &Vec4d| -> f32 {
            pn.dot(p) - pn.dot(&plane.position)
        };

        // test each point 
        for (vertex, texture) in self.p.iter().zip(self.t.iter()) {
            // Get signed distance of each point in triangle to plane
            if dist(vertex) >= 0.0 {
                inside_points[inside_count] = *vertex;
                inside_tex[inside_count] = *texture;
                inside_count += 1;
            } else {
                outside_points[outside_count] = *vertex;
                outside_tex[outside_count] = *texture;
                outside_count += 1;
            }
        }

        // Now classify triangle points, and break the input triangle into 
		// smaller output triangles if required. There are four possible
        // outcomes...
        
        match (inside_count, outside_count) {
            // zero inside - clip entire triangle
            (0, _) => Vec::<Triangle>::new(),
            // all inside - no clipping needed
            (3, _) => vec!(*self),
            // one inside, two outside - one smaller triangle
            (1, 2) => {
                let mut new_tri = Triangle::new();
                // The inside point is valid, so keep that...
                new_tri.p[0] = inside_points[0];
                new_tri.t[0] = inside_tex[0];
                new_tri.col = self.col;

                let a = plane.line_intersect_plane(inside_points[0], outside_points[0]);
                let b = plane.line_intersect_plane(inside_points[0], outside_points[1]);

                new_tri.p[1] = a.0;
                new_tri.p[2] = b.0;

                new_tri.t[1] = inside_tex[0] + ((outside_tex[0] - inside_tex[0]) * a.1);
                new_tri.t[2] = inside_tex[0] + ((outside_tex[1] - inside_tex[0]) * b.1);

                vec!(new_tri)
            },
            // two inside, one outside - clipped to a quad (2 triangles)
            (2, 1) => {
                let mut new_tri_a = Triangle::new();
                let mut new_tri_b = Triangle::new();

                // The first triangle consists of the two inside points and a new
                // point determined by the location where one side of the triangle
                // intersects with the plane
                new_tri_a.p[0] = inside_points[0];
                new_tri_a.t[0] = inside_tex[0];
                new_tri_a.col = self.col;

                new_tri_a.p[1] = inside_points[1];
                new_tri_a.t[1] = inside_tex[1];

                let a = plane.line_intersect_plane(inside_points[0], outside_points[0]);
                new_tri_a.p[2] = a.0;
                new_tri_a.t[2] = inside_tex[0] + ((outside_tex[0] - inside_tex[0]) * a.1);

                // The second triangle is composed of one of he inside points, a
                // new point determined by the intersection of the other side of the 
                // triangle and the plane, and the newly created point above
                new_tri_b.p[0] = inside_points[1];
                new_tri_b.t[0] = inside_tex[1];
                new_tri_b.p[1] = new_tri_a.p[2];
                new_tri_b.t[1] = new_tri_a.t[2];
                new_tri_b.col = self.col;

                let b = plane.line_intersect_plane(inside_points[1], outside_points[0]);
                new_tri_b.p[2] = b.0;
                new_tri_b.t[2] = inside_tex[1] + ((outside_tex[0] - inside_tex[1]) * b.1);

                vec!(new_tri_a, new_tri_b)
            },
            // not possible
            (_, _) => panic!(),
        }
    }
}