use crate::color::Color;
use crate::WHITE;
use crate::gfx3d::plane::Plane;
use crate::gfx3d::vec4d::Vec4d;
use crate::{PGE, Pixel};
use crate::gfx3d::triangle::Triangle;
use crate::Sprite;
use super::vec3d::Vec3d;
use super::mat4x4::Mat4x4;
use std::cmp::{max, min};

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

                    //self.textured_triangle(pge, tri_raster.p[0].x as i32, tri_raster.p[0].y as i32, tri_raster.t[0].x, tri_raster.t[0].y, tri_raster.t[0].z, 
                    //    tri_raster.p[1].x as i32, tri_raster.p[1].y as i32, tri_raster.t[1].x, tri_raster.t[1].y, tri_raster.t[1].z, 
                    //    tri_raster.p[2].x as i32, tri_raster.p[2].y as i32, tri_raster.t[2].x, tri_raster.t[2].y, tri_raster.t[2].z, 
                    //    &tex);

                    self.render_triangle_texture(pge, &tri_raster.p[0], &tri_raster.p[1], &tri_raster.p[2],
                        &tri_raster.t[0], &tri_raster.t[1], &tri_raster.t[2], &tex);
                        
                    tri_count += 1;

                    /*pge.draw_triangle(tri_raster.p[0].x as i32, tri_raster.p[0].y as i32, 
                        tri_raster.p[1].x as i32, tri_raster.p[1].y as i32,
                        tri_raster.p[2].x as i32, tri_raster.p[2].y as i32, 
                        &Pixel::rgb(255, 255, 255));*/
                }
            }
        }
    }

    // c1-c3 is the color for each point
    pub fn shaded_textured_triangle(&mut self, pge: &mut PGE, mut x1: i32, mut y1: i32, mut u1: f32, mut v1: f32, mut w1: f32, mut c1: Color,
        mut x2: i32, mut y2: i32, mut u2: f32, mut v2: f32, mut w2: f32, mut c2: Color,
        mut x3: i32, mut y3: i32, mut u3: f32, mut v3: f32, mut w3: f32, mut c3: Color, tex: &Sprite) {
        if y2 < y1
		{
			std::mem::swap(&mut y1, &mut y2);
			std::mem::swap(&mut x1, &mut x2);
			std::mem::swap(&mut u1, &mut u2);
			std::mem::swap(&mut v1, &mut v2);
            std::mem::swap(&mut w1, &mut w2);
            std::mem::swap(&mut c1, &mut c2);
		}
		if y3 < y1
		{
			std::mem::swap(&mut y1, &mut y3);
			std::mem::swap(&mut x1, &mut x3);
			std::mem::swap(&mut u1, &mut u3);
			std::mem::swap(&mut v1, &mut v3);
            std::mem::swap(&mut w1, &mut w3);
            std::mem::swap(&mut c1, &mut c3);
		}
		if y3 < y2
		{
			std::mem::swap(&mut y2, &mut y3);
			std::mem::swap(&mut x2, &mut x3);
			std::mem::swap(&mut u2, &mut u3);
			std::mem::swap(&mut v2, &mut v3);
            std::mem::swap(&mut w2, &mut w3);
            std::mem::swap(&mut c2, &mut c3);
		}

		let mut dy1 = y2 - y1;
		let mut dx1 = x2 - x1;
		let mut dv1 = v2 - v1;
		let mut du1 = u2 - u1;
        let mut dw1 = w2 - w1;
        let mut dc1 = c2 - c1;

		let dy2 = y3 - y1;
		let dx2 = x3 - x1;
		let dv2 = v3 - v1;
		let du2 = u3 - u1;
        let dw2 = w3 - w1;
        let dc2 = c3 - c1;

        let mut tex_u: f32;
        let mut tex_v: f32; 
        let mut tex_w: f32;
        let mut col_r: f32;
        let mut col_g: f32;
        let mut col_b: f32;
        let mut col_a: f32;

		let mut dax_step = 0.0; 
        let mut du1_step = 0.0; 
        let mut du2_step = 0.0; 
        let mut dw1_step = 0.0; 
        let mut dc1r_step = 0.0;
        let mut dc1g_step = 0.0;
        let mut dc1b_step = 0.0;
        let mut dc1a_step = 0.0;
        let mut dbx_step = 0.0;
        let mut dv1_step = 0.0;
        let mut dv2_step = 0.0;
        let mut dw2_step = 0.0;
        let mut dc2r_step = 0.0;
        let mut dc2g_step = 0.0;
        let mut dc2b_step = 0.0;
        let mut dc2a_step = 0.0;

		if dy1 > 0 {
            dax_step = dx1 as f32 / dy1.abs() as f32;
		    du1_step = du1 / dy1.abs() as f32;
		    dv1_step = dv1 / dy1.abs() as f32;
            dw1_step = dw1 / dy1.abs() as f32;
            dc1r_step = dc1.r / dy1.abs() as f32;
            dc1g_step = dc1.g / dy1.abs() as f32;
            dc1b_step = dc1.b / dy1.abs() as f32;
            dc1a_step = dc1.a / dy1.abs() as f32;
        }

		if dy2 > 0 {
            dbx_step = dx2 as f32 / dy2.abs() as f32;
            du2_step = du2 / dy2.abs() as f32;
		    dv2_step = dv2 / dy2.abs() as f32;
            dw2_step = dw2 / dy2.abs() as f32;
            dc2r_step = dc2.r / dy2.abs() as f32;
            dc2g_step = dc2.g / dy2.abs() as f32;
            dc2b_step = dc2.b / dy2.abs() as f32;
            dc2a_step = dc2.a / dy2.abs() as f32;
        }

		if dy1 > 0 {
            for i in y1..y2 {
				let mut ax = x1 + ((i - y1) as f32 * dax_step) as i32;
				let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

				let mut tex_su = u1 + (i - y1) as f32 * du1_step;
				let mut tex_sv = v1 + (i - y1) as f32 * dv1_step;
				let mut tex_sw = w1 + (i - y1) as f32 * dw1_step;
				let mut col_sr = c1.r + (i - y1) as f32 * dc1r_step;
				let mut col_sg = c1.g + (i - y1) as f32 * dc1g_step;
				let mut col_sb = c1.b + (i - y1) as f32 * dc1b_step;
				let mut col_sa = c1.a + (i - y1) as f32 * dc1a_step;

				let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
				let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
                let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;
                let mut col_er = c1.r + (i - y1) as f32 * dc2r_step;
				let mut col_eg = c1.g + (i - y1) as f32 * dc2g_step;
				let mut col_eb = c1.b + (i - y1) as f32 * dc2b_step;
				let mut col_ea = c1.a + (i - y1) as f32 * dc2a_step;

				if ax > bx {
					std::mem::swap(&mut ax, &mut bx);
					std::mem::swap(&mut tex_su, &mut tex_eu);
					std::mem::swap(&mut tex_sv, &mut tex_ev);
					std::mem::swap(&mut tex_sw, &mut tex_ew);
					std::mem::swap(&mut col_sr, &mut col_er);
					std::mem::swap(&mut col_sg, &mut col_eg);
					std::mem::swap(&mut col_sb, &mut col_eb);
					std::mem::swap(&mut col_sa, &mut col_ea);
				}

				let tstep = 1.0 / (bx - ax) as f32;
				let mut t = 0.0;

                for j in ax..bx {
					tex_u = (1.0 - t) * tex_su + t * tex_eu;
					tex_v = (1.0 - t) * tex_sv + t * tex_ev;
					tex_w = (1.0 - t) * tex_sw + t * tex_ew;
					col_r = (1.0 - t) * col_sr + t * col_er;
					col_g = (1.0 - t) * col_sg + t * col_eg;
					col_b = (1.0 - t) * col_sb + t * col_eb;
                    col_a = (1.0 - t) * col_sa + t * col_ea;
                    // TODO - use interpolated color!!!!
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
		dc1 = c3 - c2;

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
            for i in y2..y3 {
				let mut ax = x2 + ((i - y2) as f32 * dax_step) as i32;
				let mut bx = x1 + ((i - y1) as f32 * dbx_step) as i32;

				let mut tex_su = u2 + (i - y2) as f32 * du1_step;
				let mut tex_sv = v2 + (i - y2) as f32 * dv1_step;
                let mut tex_sw = w2 + (i - y2) as f32 * dw1_step;
                let mut col_sr = c2.r + (i - y2) as f32 * dc1r_step;
				let mut col_sg = c2.g + (i - y2) as f32 * dc1g_step;
				let mut col_sb = c2.b + (i - y2) as f32 * dc1b_step;
				let mut col_sa = c2.a + (i - y2) as f32 * dc1a_step;

				let mut tex_eu = u1 + (i - y1) as f32 * du2_step;
				let mut tex_ev = v1 + (i - y1) as f32 * dv2_step;
                let mut tex_ew = w1 + (i - y1) as f32 * dw2_step;
                let mut col_er = c1.r + (i - y1) as f32 * dc2r_step;
				let mut col_eg = c1.g + (i - y1) as f32 * dc2g_step;
				let mut col_eb = c1.b + (i - y1) as f32 * dc2b_step;
				let mut col_ea = c1.a + (i - y1) as f32 * dc2a_step;

				if ax > bx {
					std::mem::swap(&mut ax, &mut bx);
					std::mem::swap(&mut tex_su, &mut tex_eu);
					std::mem::swap(&mut tex_sv, &mut tex_ev);
                    std::mem::swap(&mut tex_sw, &mut tex_ew);
                    std::mem::swap(&mut col_sr, &mut col_er);
					std::mem::swap(&mut col_sg, &mut col_eg);
					std::mem::swap(&mut col_sb, &mut col_eb);
					std::mem::swap(&mut col_sa, &mut col_ea);
				}

				let tstep = 1.0 / (bx - ax) as f32;
				let mut t = 0.0;

                for j in ax..bx {
					tex_u = (1.0 - t) * tex_su + t * tex_eu;
					tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                    tex_w = (1.0 - t) * tex_sw + t * tex_ew;
                    col_r = (1.0 - t) * col_sr + t * col_er;
					col_g = (1.0 - t) * col_sg + t * col_eg;
					col_b = (1.0 - t) * col_sb + t * col_eb;
                    col_a = (1.0 - t) * col_sa + t * col_ea;
                    // TODO - use interpolated color!!!!

                    if tex_w > self.depth_buffer[(i * pge.screen_width + j) as usize] {
						pge.draw(j, i, &tex.sample(tex_u / tex_w, tex_v / tex_w));
						self.depth_buffer[(i * pge.screen_width + j) as usize] = tex_w;
					}
					t += tstep;
				}
			}
		}
    }

    pub fn render_triangle_texture(&mut self, pge: &mut PGE, a: &Vec4d, b: &Vec4d, c: &Vec4d, 
        auv: &Vec3d, buv: &Vec3d, cuv: &Vec3d,
        tex: &Sprite) {
        // algorithm only fills counter clockwise triangles, so swap as needed
        // For a triangle A B C, you can find the winding by computing the cross product (B - A) x (C - A). For 2d tri's, with z=0, it will only have a z component.
        // To give all the same winding, swap vertices C and B if this z component is negative.
        let cross = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y); 
        let v0 = a;
        let mut v1 = b;
        let mut v2 = c;
        let v0uv = auv;
        let mut v1uv = buv;
        let mut v2uv = cuv;
        if cross > 0.0 { 
            std::mem::swap(&mut v1, &mut v2);
            std::mem::swap(&mut v1uv, &mut v2uv);
        }

        // use fixed-point only for X and Y.  Avoid work for Z and W.
        let fxPtX0 = (v0.x + 0.5) as i32;
        let fxPtX1 = (v1.x + 0.5) as i32;
        let fxPtX2 = (v2.x + 0.5) as i32;
        let fxPtY0 = (v0.y + 0.5) as i32;
        let fxPtY1 = (v1.y + 0.5) as i32;
        let fxPtY2 = (v2.y + 0.5) as i32;
        let Z0 = v0.z;
        let mut Z1 = v1.z;
        let mut Z2 = v2.z;

        // texture space
        let t0u = v0uv.x;
        let mut t1u = v1uv.x;
        let mut t2u = v2uv.x;

        let t0v = v0uv.y;
        let mut t1v = v1uv.y;
        let mut t2v = v2uv.y;

        let t0z = v0uv.z;
        let mut t1z = v1uv.z;
        let mut t2z = v2uv.z;

        // Fab(x, y) =     Ax       +       By     +      C              = 0
        // Fab(x, y) = (ya - yb)x   +   (xb - xa)y + (xa * yb - xb * ya) = 0
        // Compute A = (ya - yb) for the 3 line segments that make up each triangle
        let A0 = fxPtY1 - fxPtY2;
        let A1 = fxPtY2 - fxPtY0;
        let A2 = fxPtY0 - fxPtY1;

        // Compute B = (xb - xa) for the 3 line segments that make up each triangle
        let B0 = fxPtX2 - fxPtX1;
        let B1 = fxPtX0 - fxPtX2;
        let B2 = fxPtX1 - fxPtX0;

        // Compute C = (xa * yb - xb * ya) for the 3 line segments that make up each triangle
        let C0 = fxPtX1 * fxPtY2 - fxPtX2 * fxPtY1;
        let C1 = fxPtX2 * fxPtY0 - fxPtX0 * fxPtY2;
        let C2 = fxPtX0 * fxPtY1 - fxPtX1 * fxPtY0;

        // Determine edges
        let is_top_left = |v0: &Vec4d, v1: &Vec4d| -> bool {
            v0.y > v1.y
        };

        // We follow fill rules and add a bias
        let bias0 = if is_top_left(v1, v2) { 0 } else { -1 };
        let bias1 = if is_top_left(v2, v0) { 0 } else { -1 };
        let bias2 = if is_top_left(v0, v1) { 0 } else { -1 };

        // Compute triangle area
        let tri_area = (fxPtX1 - fxPtX0) * (fxPtY2 - fxPtY0) - (fxPtX0 - fxPtX2) * (fxPtY0 - fxPtY1);
        let one_over_tri_area = 1.0 / tri_area as f32;

        Z1 = (Z1 - Z0) * one_over_tri_area;
        Z2 = (Z2 - Z0) * one_over_tri_area;

        t1u = (t1u - t0u) * one_over_tri_area;
        t2u = (t2u - t0u) * one_over_tri_area;

        t1v = (t1v - t0v) * one_over_tri_area;
        t2v = (t2v - t0v) * one_over_tri_area;

        t1z = (t1z - t0z) * one_over_tri_area;
        t2z = (t2z - t0z) * one_over_tri_area;

        // Use bounding box traversal strategy to determine which pixels to rasterize 
        let startX = max(min(min(fxPtX0, fxPtX1), fxPtX2), 0);// & 0xFFFFFFFE;
        let endX = min(max(max(fxPtX0, fxPtX1), fxPtX2), pge.screen_width);

        let startY = max(min(min(fxPtY0, fxPtY1), fxPtY2), 0);// & 0xFFFFFFFE;
        let endY = min(max(max(fxPtY0, fxPtY1), fxPtY2), pge.screen_height);

        let mut rowIdx = startY * pge.screen_width + startX;
        let col = startX;
        let mut row = startY;

        // Incrementally compute Fab(x, y) for all the pixels inside the bounding box formed by (startX, endX) and (startY, endY)
        let mut alpha0 = (A0 * col) + (B0 * row) + C0 + bias0;
        let mut beta0 = (A1 * col) + (B1 * row) + C1 + bias1;
        let mut gama0 = (A2 * col) + (B2 * row) + C2 + bias2;

        let zx = A1 as f32 * Z1 + A2 as f32 * Z2;

        let tux = A1 as f32 * t1u + A2 as f32 * t2u;
        let tvx = A1 as f32 * t1v + A2 as f32 * t2v;
        let tz = A1 as f32 * t1z + A2 as f32 * t2z;

        for _ in startY..endY {
            // Compute barycentric coordinates 
            let mut index = rowIdx;
            let mut alpha = alpha0;
            let mut beta = beta0;
            let mut gama = gama0;

            let mut depth = Z0 + Z1 * beta as f32 + Z2 * gama as f32;

            let mut u = t0u + t1u * beta as f32 + t2u * gama as f32;
            let mut v = t0v + t1v * beta as f32 + t2v * gama as f32;
            let mut uv_z = t0z + t1z * beta as f32 + t2z * gama as f32;

            for _ in startX..endX {
                //Test Pixel inside triangle
                let mask = alpha | beta | gama;

                let previousDepthValue = self.depth_buffer[index as usize];
                let mergedDepth = depth.max(previousDepthValue);				
                let finaldepth = if mask < 0 { previousDepthValue } else { mergedDepth };

                self.depth_buffer[index as usize] = finaldepth;

                if mask > 0 && previousDepthValue < finaldepth {
                    let one_over_uv_z = 1.0 / uv_z;
                    let sample = tex.sample(u * one_over_uv_z, v * one_over_uv_z);
                    pge.draw_target[pge.current_draw_target].data[index as usize] = sample;
                }

                // inc per pixel
                index += 1;
                alpha += A0;
                beta += A1;
                gama += A2;
                depth += zx;
                u += tux;
                v += tvx;
                uv_z += tz;
            }

            // inc per row
            row += 1;
            rowIdx += pge.screen_width;
            alpha0 += B0;
            beta0 += B1;
            gama0 += B2;
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
            for i in y1..y2 {
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
					let index = (i * pge.screen_width + j) as usize;
                    if index < self.depth_buffer.len() {
                        if tex_w > self.depth_buffer[index] {
                            pge.draw(j, i, &tex.sample(tex_u / tex_w, tex_v / tex_w));
                            self.depth_buffer[index] = tex_w;
                        }
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
            for i in y2..y3 {
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

                    let index = (i * pge.screen_width + j) as usize;
                    if index < self.depth_buffer.len() {
                        if tex_w > self.depth_buffer[index] {
                            pge.draw(j, i, &tex.sample(tex_u / tex_w, tex_v / tex_w));
                            self.depth_buffer[index] = tex_w;
                        }
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