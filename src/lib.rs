use std::{cmp::{max, min}, rc::Rc};

use layer::Layer;
use miniquad::*;
use glam::*;
//use renderable::Renderable;
pub use sprite::*;
pub use decal::*;

mod layer;
mod sprite;
mod decal;

#[allow(unused_variables)]
pub trait GameLoop {
    type GameType;
    
    fn init(pge: &mut PGE) -> Self::GameType where Self: Sized;
    fn update(&mut self, pge: &mut PGE, dt: f64) {}
    fn fixed_update(&mut self, pge: &mut PGE, dt: f64) {}
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

type Color = Pixel;

impl Pixel {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Pixel{r, g, b, a:255}
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel{r, g, b, a}
    }

    // HACK?
    pub fn from_rgba_to_bgra(&mut self) {
        std::mem::swap(&mut self.b, &mut self.r);
    }
}

// Colors
pub const WHITE: Pixel                  = Pixel{r:255, g:255, b:255, a:255};
pub const GREY: Pixel                   = Pixel{r:192, g:192, b:192, a:255};
pub const DARK_GREY: Pixel              = Pixel{r:128, g:128, b:128, a:255};
pub const VERY_DARK_GREY: Pixel         = Pixel{r:64, g:64, b:64, a:255};
pub const RED: Pixel                    = Pixel{r:255, g:0, b:0, a:255};
pub const DARK_RED: Pixel               = Pixel{r:128, g:0, b:0, a:255};
pub const VERY_DARK_RED: Pixel          = Pixel{r:64, g:0, b:0, a:255};
pub const YELLOW: Pixel                 = Pixel{r:255, g:255, b:0, a:255};
pub const DARK_YELLOW: Pixel            = Pixel{r:128, g:128, b:0, a:255};
pub const VERY_DARK_YELLOW: Pixel       = Pixel{r:64, g:64, b:0, a:255};
pub const GREEN: Pixel                  = Pixel{r:0, g:255, b:0, a:255};
pub const DARK_GREEN: Pixel             = Pixel{r:0, g:128, b:0, a:255};
pub const VERY_DARK_GREEN: Pixel        = Pixel{r:0, g:64, b:0, a:255};
pub const CYAN: Pixel                   = Pixel{r:0, g:255, b:255, a:255};
pub const DARK_CYAN: Pixel              = Pixel{r:0, g:128, b:128, a:255};
pub const VERY_DARK_CYAN: Pixel         = Pixel{r:0, g:64, b:64, a:255};
pub const BLUE: Pixel                   = Pixel{r:0, g:0, b:255, a:255};
pub const DARK_BLUE: Pixel              = Pixel{r:0, g:0, b:128, a:255};
pub const VERY_DARK_BLUE: Pixel         = Pixel{r:0, g:0, b:64, a:255};
pub const MAGENTA: Pixel                = Pixel{r:255, g:0, b:255, a:255};
pub const DARK_MAGENTA: Pixel           = Pixel{r:128, g:0, b:128, a:255};
pub const VERY_DARK_MAGENTA: Pixel      = Pixel{r:64, g:0, b:64, a:255};
pub const BLACK: Pixel                  = Pixel{r:0, g:0, b:0, a:255};
pub const BLANK: Pixel                  = Pixel{r:0, g:0, b:0, a:0};

pub enum PixelMode {
    Normal, Mask, Alpha, Custom
}

pub struct Renderable {
    sprite: SpriteRef,
    decal: Decal,
}

pub struct PGE {
    pub screen_width: usize,
    pub screen_height: usize,
    pub pixel_mode: PixelMode,
    pub blend_factor: f32,
    pub func_pixel_mode: Option<fn(x: i32, y: i32, p1: &Pixel, p2: &Pixel)>,
    pub font: Sprite,

    /// Engine internal stuff
    layers: Vec<Layer>,
    current_layer: usize,
    //keyboard_map: HashMap<> TODO:
    pixel_width: i32,
    pixel_height: i32,
    pipeline: Pipeline,
    bindings: Bindings,
    ctx: Box<dyn RenderingBackend>,
    inv_screen_size: Vec2,

    // timing stuff
    accumulator: f64,
    current_time: f64,
    pub dt: f64,
    pub time: f64,
    pub frames: usize,
    pub fixed_frames: usize,

    // input stuff
    pub mouse_pos: IVec2,
}

impl PGE {
    pub fn construct<GT>(app_name: &str, width: usize, height: usize, pix_width: usize, pix_height: usize) 
        where GT: GameLoop<GameType = GT> + 'static 
        {
        let mut conf = conf::Conf::default();

        conf.window_title = app_name.to_owned();
        conf.window_width = (width * pix_width) as i32;
        conf.window_height = (height * pix_height) as i32;
        conf.window_resizable = false;

        miniquad::start(conf, move || {
            Box::new(App::<GT> {
                game: None,
                pge: PGE::new(width, height, pix_width, pix_height),
            })
        });
    }

    fn new(width: usize, height: usize, pix_width: usize, pix_height: usize) -> Self {

        let mut ctx = window::new_rendering_backend();
        let back_buffer = Sprite::new(width as u32, height as u32);

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 1. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 0. } },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bb_sprite_ref = SpriteRef::new(back_buffer);
        let len = bb_sprite_ref.get_data_len();

        let bb_texture = ctx.new_texture_from_rgba8(width as u16, height as u16, unsafe {
            std::slice::from_raw_parts(bb_sprite_ref.get_data_ptr(), len * 4)
        });

        ctx.texture_set_filter(bb_texture, FilterMode::Nearest, MipmapFilterMode::None);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![bb_texture],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
                    vertex: shader::GL_VERTEX,
                    fragment: shader::GL_FRAGMENT,
                },
                shader::meta(),
            )
            .unwrap();
        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
        );

        let bb_sprite_ref2 = Rc::downgrade(&bb_sprite_ref.0);


        PGE { 
            screen_width: width, 
            screen_height: height, 
            pixel_width: pix_width as i32, 
            pixel_height: pix_height as i32, 
            pipeline, bindings, 
            pixel_mode: PixelMode::Normal, 
            blend_factor: 1.0, 
            func_pixel_mode: None, 
            font: PGE::construct_font_sheet(),
            layers: vec![ 
                Layer { 
                    offset: Vec2::ZERO, 
                    scale: Vec2::ONE, 
                    show: true, 
                    update: true, 
                    surface: Renderable { 
                        sprite: bb_sprite_ref, 
                        decal: Decal { 
                            sprite: bb_sprite_ref2,
                            uv_scale: Vec2::ONE,
                            width: width as u32,
                            height: height as u32,
                        }},
                    decal_instances: vec![], 
                    tint: BLANK, 
                    id: 0 }
                ], 
            current_layer: 0,
            ctx,
            accumulator: 0.0,
            current_time: date::now(),
            dt: 1.0 / 60.0,
            time: 0.0,
            frames: 0,
            fixed_frames: 0,
            mouse_pos: IVec2::ZERO,
            inv_screen_size: vec2(1.0 / width as f32, 1.0 / height as f32)
        }
    }

    pub fn set_draw_target(&mut self, layer: usize, dirty: bool) {
        if layer < self.layers.len() {
            // we could set a draw target, but instead just keep track of the layer
            self.layers[layer].update = dirty;
            self.current_layer = layer;
        }
    }

    pub fn get_mouse_x(&mut self) -> i32 {
        self.mouse_pos.x
    }

    pub fn get_mouse_y(&mut self) -> i32 {
        self.mouse_pos.y
    }

    //pub fn create_decal(&mut self, sprite: &Sprite) -> Decal {
    //    let id = self.create_texture(sprite.width, sprite.height);
    //    self.update_texture(id, &sprite);
    //    Decal { id, uv_scale: Vec2::ONE, width: sprite.width, height: sprite.height }
    //}

    //pub fn create_renderable(&mut self, width: u32, height: u32, filter: bool, clamp: bool) -> Renderable {
    //    let sprite = Sprite::new(width, height);
    //    let decal = self.create_decal(&sprite);
    //    Renderable { sprite, decal }
    //}

    pub fn create_texture(&mut self, width: u32, height: u32) -> TextureId {
        let texture = self.ctx.new_texture(
            TextureAccess::Static, 
            TextureSource::Empty, 
            TextureParams { 
                kind: TextureKind::Texture2D, 
                format: TextureFormat::RGBA8, 
                wrap: TextureWrap::Clamp, 
                min_filter: FilterMode::Linear, 
                mag_filter: FilterMode::Linear, 
                mipmap_filter: MipmapFilterMode::Linear, 
                width: width, 
                height: height, 
                allocate_mipmaps: false 
            });
        texture
    }

    pub fn update_texture(&mut self, id: TextureId, sprite: &Sprite) {
        self.ctx.texture_update(id, unsafe {
            std::slice::from_raw_parts(sprite.pixel_data.as_ptr() as *const u8, sprite.pixel_data.len() * 4)
        });
    }

    pub fn read_texture(&mut self, id: TextureId, sprite: &mut Sprite) {
        let (tw, th) = self.ctx.texture_size(id);
        if tw == sprite.width && th == sprite.height {
            let bytes: &mut [u8] = unsafe {
                std::slice::from_raw_parts_mut(sprite.pixel_data.as_ptr() as *mut u8, sprite.pixel_data.len() * 4)
            };
            self.ctx.texture_read_pixels(id, bytes);
        }
    }

    pub fn delete_texture(&mut self, id: TextureId) {
        self.ctx.delete_texture(id);
    }

    //pub fn create_layer(&mut self) -> usize {
    //    let layer = Layer { 
    //        offset: Vec2::ZERO, 
    //        scale: Vec2::ONE, 
    //        show: false, 
    //        update: false, 
    //        surface: self.create_renderable(self.screen_width as u32, self.screen_height as u32, false, true), 
    //        decal_instances: vec![], 
    //        tint: WHITE, 
    //        id: self.layers.len()
    //    };
    //    self.layers.push(layer);
    //    return self.layers.len() - 1;
    //}

    //pub fn draw_decal(&mut self, pos: Vec2, decal: &Decal, scale: Vec2, tint: &Color) {
    //    let screen_space_pos = vec2(
    //        (pos.x * self.inv_screen_size.x) * 2.0 - 1.0, 
    //        (pos.x * self.inv_screen_size.x) * 2.0 - 1.0);
    //    let screen_space_dim = vec2(
    //        screen_space_pos.x + (2.0 * decal.width as f32 * self.inv_screen_size.x) * scale.x,
    //        screen_space_pos.y - (2.0 * decal.height as f32 * self.inv_screen_size.y) * scale.y,
    //    );
//
    //    let di = DecalInstance { 
    //        pos: vec![screen_space_pos, vec2(screen_space_pos.x, screen_space_dim.y), screen_space_dim, vec2(screen_space_dim.x, screen_space_pos.y)], 
    //        uv: vec![vec2(0., 0.), vec2(0., 1.), vec2(1., 1.), vec2(1., 0.)], 
    //        w: vec![1.0, 1.0, 1.0, 1.0], 
    //        tint: *tint, 
    //        mode: DecalMode::Normal,    // TODO: get this from decal mode
    //        structure: DecalStructure::Fan  // TODO:  
    //    };
//
    //    self.layers[self.current_layer].decal_instances.push(di);
    //}

    // render the layers surface and all decal instances
    fn render_layer(&mut self) {

    }

    #[inline]
    pub fn draw(&mut self, x: i32, y: i32, p: &Pixel) {
        if self.current_layer < self.layers.len() {
            match self.pixel_mode {
                PixelMode::Normal => {
                    self.layers[self.current_layer].surface.sprite.set_pixel(x, y, p);
                }
                PixelMode::Mask => {
                    if p.a == 255 {
                        self.layers[self.current_layer].surface.sprite.set_pixel(x, y, p);
                    }
                },
                PixelMode::Alpha => {
                    let d = self.layers[self.current_layer].surface.sprite.get_pixel(x, y);
                    let a = (p.a as f32 / 255.0) * self.blend_factor;
                    let c = 1.0 - a;
                    // cheat: use fused multiply add
                    let r = a.mul_add(p.r as f32, c * d.r as f32);
                    let g = a.mul_add(p.g as f32, c * d.g as f32);
                    let b = a.mul_add(p.b as f32, c * d.b as f32);
                    self.layers[self.current_layer].surface.sprite.set_pixel(x, y, &Pixel::rgb(r as u8, g as u8, b as u8));
                },
                PixelMode::Custom => {
                    if let Some(fpm) = self.func_pixel_mode {
                        fpm(x, y, &self.layers[self.current_layer].surface.sprite.get_pixel(x, y), p);
                    }
                }
            }
        }
    }

    #[rustfmt::skip]
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, p: &Pixel) {
        let mut x = x1;
        let mut y = y1;
        let dx = i32::abs(x2 - x1);
        let dy = i32::abs(y2 - y1);
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            self.draw(x, y, p);
            if sx > 0 && sy > 0 { if x >= x2 && y >= y2 { break }};
            if sx > 0 && sy < 0 { if x >= x2 && y <= y2 { break }};
            if sx < 0 && sy > 0 { if x <= x2 && y >= y2 { break }};
            if sx < 0 && sy < 0 { if x <= x2 && y <= y2 { break }};
            if err > -dx {err = err - dy; x = x + sx; }
            if err < dy  {err = err + dx; y = y + sy; }        
        }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, p: &Pixel) {
        let mut x0 = 0;
		let mut y0 = radius;
		let mut d = 3 - 2 * radius;
		if radius <= 0 { return }

		while y0 >= x0 // only formulate 1/8 of circle
		{
			self.draw(x - x0, y - y0, p);//upper left left
			self.draw(x - y0, y - x0, p);//upper upper left
			self.draw(x + y0, y - x0, p);//upper upper right
			self.draw(x + x0, y - y0, p);//upper right right
			self.draw(x - x0, y + y0, p);//lower left left
			self.draw(x - y0, y + x0, p);//lower lower left
			self.draw(x + y0, y + x0, p);//lower lower right
			self.draw(x + x0, y + y0, p);//lower right right
			if d < 0 { d += 4 * x0 + 6; x0 += 1; }
			else { x0 += 1; y0 -= 1; d += 4 * (x0 - y0) + 10; }
		}
    }

    pub fn fill_circle(&mut self, x: i32, y: i32, radius: i32, p: &Pixel) {
        let mut x0 = 0;
		let mut y0 = radius;
		let mut d = 3 - 2 * radius;
        if radius <= 0 { return }

        let mut scanline_rendered: [bool; 2048] = [false; 2048];

		while y0 >= x0
		{
            // Modified to draw scan-lines instead of edges
            if y-y0 > 0 && y-y0 < self.screen_height as i32 {
                if scanline_rendered[(y - y0) as usize] == false {
                    self.draw_line(x - x0, y - y0, x + x0, y - y0, p);
                    scanline_rendered[(y - y0) as usize] = true;
                }
            }
            if y-x0 > 0 && y-x0 < self.screen_height as i32 {
                if scanline_rendered[(y - x0) as usize] == false {
                    self.draw_line(x - y0, y - x0, x + y0, y - x0, p);
                    scanline_rendered[(y - x0) as usize] = true;
                }
            }
            if y+y0 > 0 && y+y0 < self.screen_height as i32 {
                if scanline_rendered[(y + y0) as usize] == false {
                    self.draw_line(x - x0, y + y0, x + x0, y + y0, p);
                    scanline_rendered[(y + y0) as usize] = true;
                }
            }
            if y+x0 > 0 && y+x0 < self.screen_height as i32 {
                if scanline_rendered[(y + x0) as usize] == false {
                    self.draw_line(x - y0, y + x0, x + y0, y + x0, p);
                    scanline_rendered[(y + x0) as usize] = true;
                }
            }

			if d < 0 { x0 += 1; d += 4 * x0 + 6;  }
			else { x0 += 1; y0 -= 1; d += 4 * (x0 - y0) + 10; }
		}
    }

    pub fn draw_rect(&mut self, x: i32, y: i32, w: i32, h: i32, p: &Pixel) {
        self.draw_line(x, y, x+w, y, p);
		self.draw_line(x+w, y, x+w, y+h, p);
		self.draw_line(x+w, y+h, x, y+h, p);
		self.draw_line(x, y+h, x, y, p);
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, p: &Pixel) {
        let mut x1 = x;
        let mut y1 = y;
        let mut x2 = x + w;
		let mut y2 = y + h;

		if x < 0 { x1 = 0; }
		if x >= self.screen_width as i32 { x1 = self.screen_width as i32; }
		if y < 0 { y1 = 0; }
		if y >= self.screen_height as i32 { y1 = self.screen_height as i32; }

		if x2 < 0 { x2 = 0; }
		if x2 >= self.screen_width as i32 { x2 = self.screen_width as i32; }
		if y2 < 0 { y2 = 0; }
		if y2 >= self.screen_height as i32 { y2 = self.screen_height as i32; }

		for j in y1..y2 {
			for i in x1..x2 {
				self.draw(i, j, p);
            }
        }
    }

    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: &Pixel) {
        self.draw_line(x1, y1, x2, y2, p);
		self.draw_line(x2, y2, x3, y3, p);
		self.draw_line(x3, y3, x1, y1, p);
    }

    pub fn fill_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: &Pixel) {
        // we use tuples for this for now
        let v0 = (x1, y1);
        let mut v1 = (x2, y2);
        let mut v2 = (x3, y3);

        // algorithm only fills counter clockwise triangles, so swap as needed
        // For a triangle A B C, you can find the winding by computing the cross product (B - A) x (C - A). For 2d tri's, with z=0, it will only have a z component.
        // To give all the same winding, swap vertices C and B if this z component is negative.
        let cross = (v1.1 - v0.1) * (v2.0 - v1.0) - (v1.0 - v0.0) * (v2.1 - v1.1); 
        if cross > 0 { std::mem::swap(&mut v1, &mut v2) }
        
        // Compute triangle bounding box and clip to screen bounds
        let min_x = max(min(min(v0.0, v1.0), v2.0), 0);
        let max_x = min(max(max(v0.0, v1.0), v2.0), self.screen_width as i32 - 1);
        let min_y = max(min(min(v0.1, v1.1), v2.1), 0);
        let max_y = min(max(max(v0.1, v1.1), v2.1), self.screen_height as i32 - 1);

        // Triangle setup
        let a01 = v0.1 - v1.1;
        let b01 = v1.0 - v0.0;
        let a12 = v1.1 - v2.1;
        let b12 = v2.0 - v1.0;
        let a20 = v2.1 - v0.1;
        let b20 = v0.0 - v2.0;

        // Determine edges
        let is_top_left = |v0: (i32, i32), v1: (i32, i32)| -> bool {
            v0.1 > v1.1 
        };

        // We follow fill rules and add a bias
        let bias0 = if is_top_left(v1, v2) { 0 } else { -1 };
        let bias1 = if is_top_left(v2, v0) { 0 } else { -1 };
        let bias2 = if is_top_left(v0, v1) { 0 } else { -1 };

        // Determine barycentric coordinates
        let orient2d = |a: (i32,i32), b: (i32,i32), c: (i32,i32)| -> i32 {
            (b.0-a.0)*(c.1-a.1) - (b.1-a.1)*(c.0-a.0)
        };

        let mut p = (min_x, min_y);
        let mut w0_row = orient2d(v1, v2, p) + bias0;
        let mut w1_row = orient2d(v2, v0, p) + bias1;
        let mut w2_row = orient2d(v0, v1, p) + bias2;

        // Rasterize
        for y in min_y..max_y {
            p.1 = y;
            // Barycentric coordinates at start of row
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

                for x in min_x..max_x {
                    p.0 = x;
                    // If p is on or inside all edges, render pixel.
                    if (w0 | w1 | w2) >= 0 {
                        self.draw(p.0, p.1, col);
                    }

                    // One step to the right
                    w0 += a12;
                    w1 += a20;
                    w2 += a01;
                }
            // One row step
            w0_row += b12;
            w1_row += b20;
            w2_row += b01;
        }
    }

    pub fn draw_sprite(&mut self, x: i32, y: i32, sprite: &Sprite, scale: usize) {
        if scale > 1 {
            for j in 0..sprite.height as i32 {
                for i in 0..sprite.width as i32 {
                    for is in 0..scale {
                        for js in 0..scale {
                            self.draw(x + (i * scale as i32) + is as i32, y + (j * scale as i32) + js as i32, &sprite.get_pixel(i, j));
                        }
                    }
                }
            }
        } else {
            for j in 0..sprite.height as i32 {
                for i in 0..sprite.width as i32 {
                    self.draw(x + i, y + j, &sprite.get_pixel(i, j));
                }
            }
        }
    }

    pub fn draw_parital_sprite(&mut self, x: i32, y: i32, sprite: &Sprite, ox: i32, oy: i32, w: i32, h: i32, scale: usize) {      
        if scale > 1 {
            for j in 0..h as i32 {
                for i in 0..w as i32 {
                    for is in 0..scale {
                        for js in 0..scale {
                            self.draw(x + (i * scale as i32) + is as i32, y + (j * scale as i32) + js as i32, &sprite.get_pixel(ox + i, oy + j));
                        }
                    }
                }
            }
        } else {
            for j in 0..h as i32 {
                for i in 0..w as i32 {
                    self.draw(x + i, y + j, &sprite.get_pixel(ox + i, oy + j));
                }
            }
        }
    }

    pub fn draw_string(&mut self, x: i32, y: i32, text: &str, col: &Pixel, scale: i32) {
        let mut sx: i32 = 0;
        let mut sy: i32 = 0;

        if col.a != 255 { self.pixel_mode = PixelMode::Alpha; }
        else { self.pixel_mode = PixelMode::Mask; }

        for c in text.chars() {
            if c == '\n' {
                sx = 0;
                sy += 8 * scale;
            } else {
                let mut ox: i32 = 15;
                let mut oy: i32 = 5;
                if c.is_ascii() {
                    ox = ((c as u32 - 32) % 16) as i32;
                    oy = ((c as u32 - 32) / 16) as i32;
                }
                if scale > 1 {
                    for j in 0..8 as i32 {
                        for i in 0..8 as i32 {
                            if self.font.get_pixel(i + ox * 8, j + oy * 8).r > 0 {
                                for js in 0..scale as i32 {
                                    for is in 0..scale as i32 {
                                        self.draw(x + sx + (i*scale) + is, y + sy + (j*scale) + js, &col);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    for j in 0..8 as i32 {
                        for i in 0..8 as i32 {
                            if self.font.get_pixel(i + ox * 8, j + oy * 8).r > 0 {
                                self.draw(x + sx + i, y + sy + j, &col);
                            }
                        }
                    }
                }
                sx += 8 * scale;
            }
        }
    }

    pub fn clear(&mut self, p: &Pixel) {
        self.layers[self.current_layer].surface.sprite.clear(*p);
    }

    fn construct_font_sheet() -> Sprite {
        let data = include_bytes!("../font.png");
        let image = image::load_from_memory_with_format(data, image::ImageFormat::Png).unwrap();
        let raw_image = image.as_bytes();
        Sprite::new_with_data(128, 48, raw_image)
    }

    fn render_decal_instance(&mut self) {

    }

    pub fn render(&mut self) {
        self.ctx.texture_update(self.bindings.images[0], unsafe {
            let layer = if self.current_layer < self.layers.len() { self.current_layer }
            else { 0 };
            let len = self.layers[layer].surface.sprite.get_data_len();
            std::slice::from_raw_parts(self.layers[layer].surface.sprite.get_data_ptr(), len * 4)
        });

        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();

        //
    }
}

pub struct App<T> {
    pub pge: PGE,
    pub game: Option<Box<dyn GameLoop<GameType = T>>>,
}

impl<T> EventHandler for App<T> where T: GameLoop<GameType = T> + 'static {
    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        // Mouse coords come in screen space
		// But leave in pixel space
        let x = x as i32;
        let y = y as i32;
		self.pge.mouse_pos.x = x / self.pge.pixel_width;
		self.pge.mouse_pos.y = y / self.pge.pixel_height;

		if self.pge.mouse_pos.x >= self.pge.screen_width as i32 {
			self.pge.mouse_pos.x = self.pge.screen_width as i32 - 1;
        }
		if self.pge.mouse_pos.y >= self.pge.screen_height as i32 {
			self.pge.mouse_pos.y = self.pge.screen_height as i32 - 1;
        }

		if self.pge.mouse_pos.x < 0
			{ self.pge.mouse_pos.x = 0; }
		if self.pge.mouse_pos.y < 0
			{ self.pge.mouse_pos.y = 0; }
    }

    fn update(&mut self) {
        if let Some(game) = &mut self.game {
            let new_time = date::now();
            let frame_time = new_time - self.pge.current_time;
            let dt = self.pge.dt;
            self.pge.current_time = new_time;
            self.pge.accumulator += frame_time;

            // we always call update at max frame rate
            game.update(&mut self.pge, frame_time);
            self.pge.frames += 1;

            // fixed update is only called at a fixed rate
            while self.pge.accumulator >= frame_time {
                game.fixed_update(&mut self.pge, dt);
                self.pge.accumulator -= dt;
                self.pge.time += dt;
                self.pge.fixed_frames += 1;
            }
        } else {
            self.game = Some(Box::new(T::init(&mut self.pge)))
        }
    }

    fn draw(&mut self) {
        self.pge.render();
    }
}

#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}



mod shader {
    use miniquad::*;

    pub const GL_VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        texcoord = in_uv;
    }"#;

    pub const GL_FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}