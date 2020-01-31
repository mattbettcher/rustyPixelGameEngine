use crate::WHITE;
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
    texture: Sprite,
    view_x: f32,
    view_y: f32,
    view_w: f32,
    view_h: f32,

    near: f32,

    depth_buffer: Vec<f32>,
}

impl Pipeline {
    pub fn new(width: usize, height: usize) -> Self {
        Pipeline {
            proj: Mat4x4::make_identity(),
            view: Mat4x4::make_identity(),
            world: Mat4x4::make_identity(),
            texture: Sprite::new(0, 0),
            view_x: 0.0,
            view_y: 0.0,
            view_w: 0.0,
            view_h: 0.0,

            near: 0.0,
        
            depth_buffer: vec![0.0; width * height],
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

    pub fn set_texture(&mut self, texture: &Sprite) {
        self.texture = texture.clone();
    }

    pub fn render(&mut self, pge: &mut PGE, triangles: &Vec<Triangle>, tex: &Sprite) { // flags: RenderOptions, cull_dir: CullDirection) {
        let mut tri_count = 0;
        let world_view = self.world * self.view;

        // Process Triangles
        for tri in triangles {
            let mut tri_transformed = Triangle::new();
            tri_transformed.col = tri.col;
			// Just copy through texture coordinates
            tri_transformed.t = tri.t;
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

            for near_tri in near_clipped_tris {
                let mut tri_projected = near_tri;
                tri_projected.col = near_tri.col;

                // Project new triangle
                tri_projected.p[0] = self.proj * near_tri.p[0];
                tri_projected.p[1] = self.proj * near_tri.p[1];
                tri_projected.p[2] = self.proj * near_tri.p[2];

                // Apply Projection to Verts
                tri_projected.p[0] /= tri_projected.p[0].w;
                tri_projected.p[1] /= tri_projected.p[1].w;
                tri_projected.p[2] /= tri_projected.p[2].w;

                // Apply Projection to Tex coords
                tri_projected.t[0].x /= tri_projected.p[0].w;
                tri_projected.t[1].x /= tri_projected.p[1].w;
                tri_projected.t[2].x /= tri_projected.p[2].w;

                tri_projected.t[0].y /= tri_projected.p[0].w;
                tri_projected.t[1].y /= tri_projected.p[1].w;
                tri_projected.t[2].y /= tri_projected.p[2].w;

                tri_projected.t[0].z = 1.0 / tri_projected.p[0].w;
                tri_projected.t[1].z = 1.0 / tri_projected.p[1].w;
                tri_projected.t[2].z = 1.0 / tri_projected.p[2].w;

                // Clip against viewport in screen space
				// Clip triangles against all four screen edges, this could yield
				// a bunch of triangles, so create a queue that we traverse to 
				//  ensure we only test new triangles generated against planes
                // Add initial triangle
                let mut view_clipped_tris = vec!(tri_projected);
                for p in 0..4 {
                    if let Some(tri_to_clip) = view_clipped_tris.pop() {
                        // Clip it against a plane. We only need to test each 
                        // subsequent plane, against subsequent new triangles
                        // as all triangles after a plane clip are guaranteed
                        // to lie on the inside of the plane. I like how this
                        // comment is almost completely and utterly justified
                        let clipped = match p {
                            0 => { tri_to_clip.clip_against_plane(&Plane { position: Vec4d{x: 0.0, y:-1.0, z:0.0, w:1.0}, normal: Vec4d{x: 0.0, y:1.0, z:0.0, w:1.0} }) },
                            1 => { tri_to_clip.clip_against_plane(&Plane { position: Vec4d{x: 0.0, y:1.0, z:0.0, w:1.0}, normal: Vec4d{x: 0.0, y:-1.0, z:0.0, w:1.0} }) },
                            2 => { tri_to_clip.clip_against_plane(&Plane { position: Vec4d{x: 1.0, y:0.0, z:0.0, w:1.0}, normal: Vec4d{x: -1.0, y:0.0, z:0.0, w:1.0} }) },
                            3 => { tri_to_clip.clip_against_plane(&Plane { position: Vec4d{x: -1.0, y:0.0, z:0.0, w:1.0}, normal: Vec4d{x: 1.0, y:0.0, z:0.0, w:1.0} }) },
                            _ => { panic!() }
                        };

                        for t in clipped {
                            view_clipped_tris.push(t);
                        }
                    }
                }

                for mut tri_raster in view_clipped_tris {
                    // Scale to viewport
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
                        
                    /*pge.fill_triangle(tri_raster.p[0].x as i32, tri_raster.p[0].y as i32, 
                        tri_raster.p[1].x as i32, tri_raster.p[1].y as i32,
                        tri_raster.p[2].x as i32, tri_raster.p[2].y as i32, 
                        &tri_raster.col);*/
                    
                    //tri_raster.draw_tex(pge, &tex);

                    self.textured_triangle(pge, tri_raster.p[0].x as i32, tri_raster.p[0].y as i32, tri_raster.t[0].x, tri_raster.t[0].y, tri_raster.t[0].z, 
                        tri_raster.p[1].x as i32, tri_raster.p[1].y as i32, tri_raster.t[1].x, tri_raster.t[1].y, tri_raster.t[1].z, 
                        tri_raster.p[2].x as i32, tri_raster.p[2].y as i32, tri_raster.t[2].x, tri_raster.t[2].y, tri_raster.t[2].z, 
                        &tex);
                        
                    tri_count += 1;

                    /*pge.draw_triangle(tri_raster.p[0].x as i32, tri_raster.p[0].y as i32, 
                        tri_raster.p[1].x as i32, tri_raster.p[1].y as i32,
                        tri_raster.p[2].x as i32, tri_raster.p[2].y as i32, 
                        &Pixel::rgb(255, 255, 255));*/
                }
            }
        }
    }

    pub fn textured_triangle(&mut self, pge: &mut PGE, mut x1: i32, mut y1: i32, mut u1: f32, mut v1: f32, mut w1: f32,
        mut x2: i32, mut y2: i32, mut u2: f32, mut v2: f32, mut w2: f32,
        mut x3: i32, mut y3: i32, mut u3: f32, mut v3: f32, mut w3: f32, tex: &Sprite) {
        if y2 < y1
		{
			std::mem::swap(&mut y1, &mut y2);
			std::mem::swap(&mut x1, &mut x2);
			std::mem::swap(&mut u1, &mut u2);
			std::mem::swap(&mut v1, &mut v2);
			std::mem::swap(&mut w1, &mut w2);
		}
		if y3 < y1
		{
			std::mem::swap(&mut y1, &mut y3);
			std::mem::swap(&mut x1, &mut x3);
			std::mem::swap(&mut u1, &mut u3);
			std::mem::swap(&mut v1, &mut v3);
			std::mem::swap(&mut w1, &mut w3);
		}
		if y3 < y2
		{
			std::mem::swap(&mut y2, &mut y3);
			std::mem::swap(&mut x2, &mut x3);
			std::mem::swap(&mut u2, &mut u3);
			std::mem::swap(&mut v2, &mut v3);
			std::mem::swap(&mut w2, &mut w3);
		}

		let mut dy1 = y2 - y1;
		let mut dx1 = x2 - x1;
		let mut dv1 = v2 - v1;
		let mut du1 = u2 - u1;
		let mut dw1 = w2 - w1;

		let dy2 = y3 - y1;
		let dx2 = x3 - x1;
		let dv2 = v3 - v1;
		let du2 = u3 - u1;
		let dw2 = w3 - w1;

        let mut tex_u: f32;
        let mut tex_v: f32; 
        let mut tex_w: f32;

		let mut dax_step = 0.0; 
        let mut du1_step = 0.0; 
        let mut du2_step = 0.0; 
        let mut dw1_step = 0.0; 
        let mut dbx_step = 0.0;
        let mut dv1_step = 0.0;
        let mut dv2_step = 0.0;
        let mut dw2_step = 0.0;

		if dy1 > 0 {
            dax_step = dx1 as f32 / dy1.abs() as f32;
		    du1_step = du1 / dy1.abs() as f32;
		    dv1_step = dv1 / dy1.abs() as f32;
            dw1_step = dw1 / dy1.abs() as f32;
        }

		if dy2 > 0 {
            du2_step = du2 / dy2.abs() as f32;
		    dv2_step = dv2 / dy2.abs() as f32;
            dw2_step = dw2 / dy2.abs() as f32;
            dbx_step = dx2 as f32 / dy2.abs() as f32;
        }

		if dy1 > 0 {
            for i in y1..=y2 {
				let mut ax = x1 + ((i - y1) as f32 * dax_step) as i32;
				let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

				let mut tex_su = u1 + (i - y1) as f32 * du1_step;
				let mut tex_sv = v1 + (i - y1) as f32 * dv1_step;
				let mut tex_sw = w1 + (i - y1) as f32 * dw1_step;

				let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
				let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
				let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;

				if ax > bx {
					std::mem::swap(&mut ax, &mut bx);
					std::mem::swap(&mut tex_su, &mut tex_eu);
					std::mem::swap(&mut tex_sv, &mut tex_ev);
					std::mem::swap(&mut tex_sw, &mut tex_ew);
				}

				let tstep = 1.0 / (bx - ax) as f32;
				let mut t = 0.0;

                for j in ax..bx {
					tex_u = (1.0 - t) * tex_su + t * tex_eu;
					tex_v = (1.0 - t) * tex_sv + t * tex_ev;
					tex_w = (1.0 - t) * tex_sw + t * tex_ew;
					if tex_w > self.depth_buffer[(i * pge.screen_width + j) as usize] {
						pge.draw(j, i, &tex.sample(tex_u / tex_w, tex_v / tex_w));
                        self.depth_buffer[(i * pge.screen_width + j) as usize] = tex_w;
					}
					t += tstep;
				}

			}
		}

		dy1 = y3 - y2;
		dx1 = x3 - x2;
		dv1 = v3 - v2;
		du1 = u3 - u2;
		dw1 = w3 - w2;

		if dy1 > 0 { dax_step = dx1 as f32 / dy1.abs() as f32; }
		if dy2 > 0 { dbx_step = dx2 as f32 / dy2.abs() as f32; }

        du1_step = 0.0;
        dv1_step = 0.0;
		if dy1 > 0 {
            du1_step = du1 / dy1.abs() as f32;
		    dv1_step = dv1 / dy1.abs() as f32;
            dw1_step = dw1 / dy1.abs() as f32;
        }

		if dy1 > 0 {
            for i in y2..=y3 {
				let mut ax = x2 + ((i - y2) as f32 * dax_step) as i32;
				let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

				let mut tex_su = u2 + (i - y2) as f32 * du1_step;
				let mut tex_sv = v2 + (i - y2) as f32 * dv1_step;
				let mut tex_sw = w2 + (i - y2) as f32 * dw1_step;

				let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
				let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
				let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;

				if ax > bx {
					std::mem::swap(&mut ax, &mut bx);
					std::mem::swap(&mut tex_su, &mut tex_eu);
					std::mem::swap(&mut tex_sv, &mut tex_ev);
					std::mem::swap(&mut tex_sw, &mut tex_ew);
				}

				let tstep = 1.0 / (bx - ax) as f32;
				let mut t = 0.0;

                for j in ax..bx {
					tex_u = (1.0 - t) * tex_su + t * tex_eu;
					tex_v = (1.0 - t) * tex_sv + t * tex_ev;
					tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                    if tex_w > self.depth_buffer[(i * pge.screen_width + j) as usize] {
						pge.draw(j, i, &tex.sample(tex_u / tex_w, tex_v / tex_w));
						self.depth_buffer[(i * pge.screen_width + j) as usize] = tex_w;
					}
					t += tstep;
				}
			}
		}
    }

    pub fn clear_depth(&mut self) {
        // NOTE: sloooooooow!!!!!
        /*
        for y in 0..self.screen_height {
            for x in 0..self.screen_width {
                self.draw_target[self.current_draw_target].set_pixel(x as i32, y as i32, &p);
            }
        }
        */
        // Much faster, but still might be slow?
        //self.draw_target[self.current_draw_target].clear(p.clone());
        // Proper way, adds about 30 fps
        for i in self.depth_buffer.iter_mut() { *i = 0.0; }
    }
}