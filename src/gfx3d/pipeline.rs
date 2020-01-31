use crate::gfx3d::plane::Plane;
use crate::gfx3d::vec4d::Vec4d;
use crate::{PGE, Pixel};
use crate::gfx3d::triangle::Triangle;
use crate::Sprite;
use super::vec3d::Vec3d;
use super::mat4x4::Mat4x4;

pub enum RenderOptions {
    Wire, Flat, Textured,
}

pub enum CullDirection {
    Cw, Ccw
}

pub struct Pipeline {
    proj: Mat4x4,
    view: Mat4x4,
    world: Mat4x4,
    //texture: Sprite,
    view_x: f32,
    view_y: f32,
    view_w: f32,
    view_h: f32,

    near: f32,

    depth_buffer: Vec<f32>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline {
            proj: Mat4x4::make_identity(),
            view: Mat4x4::make_identity(),
            world: Mat4x4::make_identity(),
            //texture: Sprite,
            view_x: 0.0,
            view_y: 0.0,
            view_w: 0.0,
            view_h: 0.0,

            near: 0.0,
        
            depth_buffer: vec!(),
        }
    }

    pub fn set_projection(&mut self, fov_degrees: f32, aspect_ratio: f32, near: f32, far: f32, left: f32, top: f32, width: f32, height: f32) {
        self.proj = Mat4x4::make_projection(fov_degrees, aspect_ratio, near, far);
        self.view_x = left;
        self.view_y = top;
        self.view_w = width;
        self.view_h = height;
        self.near = near;
    }

    pub fn set_camera(&mut self, pos: Vec3d, look_at: Vec3d, up: Vec3d) {
        let t = Mat4x4::make_point_at(pos, look_at, up);
        self.view = t.inverse();
    }

    pub fn set_transform(&mut self, transform: Mat4x4) {
        self.world = transform;
    }

    pub fn set_texture(&mut self, texture: Sprite) {
        // TODO
    }

    pub fn render(&mut self, pge: &mut PGE, triangles: Vec<Triangle>) { // flags: RenderOptions, cull_dir: CullDirection) {
        let mut line = 1;
        
        let world_view = self.world * self.view;
        //let mut triangles_to_render: Vec<Triangle> = Vec::new();
        // Process Triangles
        for tri in triangles {
            let mut tri_transformed = Triangle::new();
            tri_transformed.col = tri.col;
			// Just copy through texture coordinates
            //tri_transformed.t = tri.t;
			// Transform Triangle from object into projected space 
            tri_transformed.p[0] = world_view * tri.p[0];
            tri_transformed.p[1] = world_view * tri.p[1];
            tri_transformed.p[2] = world_view * tri.p[2];
            // Calculate Triangle Normal in WorldView Space
            let line_1 = tri_transformed.p[1] - tri_transformed.p[0];
            let line_2 = tri_transformed.p[2] - tri_transformed.p[0];
            let tri_world_normal = line_1.cross(&line_2).norm();
            // Cull triangles that face away from viewer
            // Clockwise only for now
            if tri_world_normal.dot(&tri_transformed.p[0]) > 0.0 { continue; }

            // Clip triangle against near plane
            let near_clipped_tris = tri_transformed.clip_against_plane(&Plane { position: Vec4d{x: 0.0, y:0.0, z:self.near, w:1.0},
                normal: Vec4d{x: 0.0, y:0.0, z:1.0, w:1.0} });

            pge.draw_string(10, 20 * line, &near_clipped_tris.len().to_string(), &Pixel::rgb(255,255,255), 2);
            line += 1;

            for near_tri in near_clipped_tris {
                let mut tri_projected = Triangle::new();
                tri_projected.col = near_tri.col;

                // Project new triangle
                tri_projected.p[0] = self.proj * near_tri.p[0];
                tri_projected.p[1] = self.proj * near_tri.p[1];
                tri_projected.p[2] = self.proj * near_tri.p[2];

                // Apply Projection to Verts
                tri_projected.p[0] /= tri_projected.p[0].w;
                tri_projected.p[1] /= tri_projected.p[1].w;
                tri_projected.p[2] /= tri_projected.p[2].w;
                
                // Scale to viewport
                let mut tri_raster = tri_projected;
                tri_raster.col = tri_projected.col;

                let mut offset_view = Vec4d { x:1.0,y:1.0,z:0.0, w:1.0 };
                tri_raster.p[0] += offset_view;
                tri_raster.p[1] += offset_view;
                tri_raster.p[2] += offset_view;
                tri_raster.p[0].x *= 0.5 * self.view_w;
                tri_raster.p[0].y *= 0.5 * self.view_h;
                tri_raster.p[1].x *= 0.5 * self.view_w;
                tri_raster.p[1].y *= 0.5 * self.view_h;
                tri_raster.p[2].x *= 0.5 * self.view_w;
                tri_raster.p[2].y *= 0.5 * self.view_h;
                offset_view = Vec4d { x:self.view_x, y:self.view_y,z:0.0, w:1.0 };
                tri_raster.p[0] += offset_view;
                tri_raster.p[1] += offset_view;
                tri_raster.p[2] += offset_view;
                    
                pge.fill_triangle(tri_raster.p[0].x as i32, tri_raster.p[0].y as i32, 
                    tri_raster.p[1].x as i32, tri_raster.p[1].y as i32,
                    tri_raster.p[2].x as i32, tri_raster.p[2].y as i32, 
                    &tri_raster.col);

                pge.draw_triangle(tri_raster.p[0].x as i32, tri_raster.p[0].y as i32, 
                    tri_raster.p[1].x as i32, tri_raster.p[1].y as i32,
                    tri_raster.p[2].x as i32, tri_raster.p[2].y as i32, 
                    &Pixel::rgb(255, 255, 255));
            }
        }
    }
}