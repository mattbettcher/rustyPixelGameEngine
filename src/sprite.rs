use crate::*;


pub enum Mode {
    Normal,
    Periodic,
    Clamp,
}

pub enum Flip {
    None,
    Horizontal,
    Vertical,
}

pub struct Sprite {
    pub width: u32,
    pub height: u32,
    pub sample_mode: Mode,
    pub pixel_data: Vec<Pixel>,
}

impl Sprite {
    pub fn new(width: u32, height: u32) -> Sprite {
        Sprite {
            width,
            height,
            sample_mode: Mode::Normal,
            pixel_data: vec![BLANK; (width * height) as usize],
        }
    }

    pub fn new_with_data(width: u32, height: u32, data: &[u8]) -> Sprite {
        unsafe {
            Sprite {
                width,
                height,
                sample_mode: Mode::Normal,
                pixel_data: std::slice::from_raw_parts(data.as_ptr() as *const Pixel, data.len() / 4).to_vec(),
            }
        }
    }

    pub fn from_rgba_to_bgra(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.pixel_data[(y * self.width + x) as usize].from_rgba_to_bgra();
            }
        }
    }

    #[inline]
    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.pixel_data[(y * self.width as i32 + x) as usize].clone()
        } else {
            Pixel::rgba(0,0,0,0)
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, p: &Pixel) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.pixel_data[(y * self.width as i32 + x) as usize] = p.clone();
        }
    }

    #[inline]
    pub fn sample(&self, x: f32, y: f32) -> Pixel {
        let sx = (x * self.width as f32) as i32;
        let sy = (y * self.height as f32) as i32;
        self.get_pixel(sx, sy)
    }

    #[inline]
    pub fn sample_bl(&self, mut u: f32, mut v: f32) -> Pixel {
        u = u * self.width as f32 - 0.5;
		v = v * self.height as f32 - 0.5;
		let x = u.floor() as i32; // cast to int rounds toward zero, not downward
		let y = v.floor() as i32; // Thanks @joshinils
		let u_ratio = u - x as f32;
		let v_ratio = v - y as f32;
		let u_opposite = 1.0 - u_ratio;
		let v_opposite = 1.0 - v_ratio;

		let p1 = self.get_pixel(x.max(0), y.max(0));
		let p2 = self.get_pixel((x+1).min(self.width as i32 - 1), y.max(0));
		let p3 = self.get_pixel(x.max(0), (y+1).min(self.height as i32 - 1));
		let p4 = self.get_pixel((x+1).min(self.width as i32 - 1), (y+1).min(self.height as i32 - 1));

		Pixel::rgb(((p1.r as f32 * u_opposite + p2.r as f32 * u_ratio) * v_opposite + (p3.r as f32 * u_opposite + p4.r as f32 * u_ratio) * v_ratio) as u8,
			       ((p1.g as f32 * u_opposite + p2.g as f32 * u_ratio) * v_opposite + (p3.g as f32 * u_opposite + p4.g as f32 * u_ratio) * v_ratio) as u8,
                   ((p1.b as f32 * u_opposite + p2.b as f32 * u_ratio) * v_opposite + (p3.b as f32 * u_opposite + p4.b as f32 * u_ratio) * v_ratio) as u8)
    }

    pub fn get_data(&self) -> &[Pixel] {
        self.pixel_data.as_slice()
    }

    pub fn clear(&mut self, p: Pixel) {
        self.pixel_data.clear();
    }
}