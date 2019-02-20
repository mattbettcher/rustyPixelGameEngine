use crate::{Sprite, PGE};

pub struct Transform2D {
    pub matrix: [[[f32; 3]; 3]; 4],
    pub target: usize,
    pub source: usize,
    pub dirty: bool,
}

impl Transform2D {
    pub fn new() -> Self {
        Transform2D {
            target: 0,
            source: 1,
            dirty: true,
            matrix: [[[0.0; 3]; 3]; 4]
        }
    }

    pub fn reset(&mut self) {
        self.target = 0;
        self.source = 1;
        self.dirty = true;

        // Columns Then Rows

		// Matrices 0 & 1 are used as swaps in Transform accumulation
		self.matrix[0][0][0] = 1.0; self.matrix[0][1][0] = 0.0; self.matrix[0][2][0] = 0.0;
		self.matrix[0][0][1] = 0.0; self.matrix[0][1][1] = 1.0; self.matrix[0][2][1] = 0.0;
		self.matrix[0][0][2] = 0.0; self.matrix[0][1][2] = 0.0; self.matrix[0][2][2] = 1.0;

		self.matrix[1][0][0] = 1.0; self.matrix[1][1][0] = 0.0; self.matrix[1][2][0] = 0.0;
		self.matrix[1][0][1] = 0.0; self.matrix[1][1][1] = 1.0; self.matrix[1][2][1] = 0.0;
		self.matrix[1][0][2] = 0.0; self.matrix[1][1][2] = 0.0; self.matrix[1][2][2] = 1.0;

		// Matrix 2 is a cache matrix to hold the immediate transform operation
		// Matrix 3 is a cache matrix to hold the inverted transform
    }

    fn multiply(&mut self) {
        for c in 0..3 {
            for r in 0..3 {
                self.matrix[self.target][c][r] = self.matrix[2][0][r] * self.matrix[self.source][c][0] +
											     self.matrix[2][1][r] * self.matrix[self.source][c][1] +
											     self.matrix[2][2][r] * self.matrix[self.source][c][2];
            }
        }

        std::mem::swap(&mut self.target, &mut self.source);
		self.dirty = true; // Any transform multiply dirties the inversion
    }

    pub fn rotate(&mut self, theta: f32) {
        // Construct Rotation Matrix
		self.matrix[2][0][0] = theta.cos();    self.matrix[2][1][0] = theta.sin(); self.matrix[2][2][0] = 0.0;
		self.matrix[2][0][1] = -theta.sin();   self.matrix[2][1][1] = theta.cos(); self.matrix[2][2][1] = 0.0;
		self.matrix[2][0][2] = 0.0;			   self.matrix[2][1][2] = 0.0;		   self.matrix[2][2][2] = 1.0;
		self.multiply();
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        // Construct Scale Matrix
		self.matrix[2][0][0] = sx;    self.matrix[2][1][0] = 0.0;   self.matrix[2][2][0] = 0.0;
		self.matrix[2][0][1] = 0.0;   self.matrix[2][1][1] = sy;    self.matrix[2][2][1] = 0.0;
		self.matrix[2][0][2] = 0.0;	  self.matrix[2][1][2] = 0.0;   self.matrix[2][2][2] = 1.0;
		self.multiply();
    }

    pub fn shear(&mut self, sx: f32, sy: f32) {
        // Construct Shear Matrix
		self.matrix[2][0][0] = 1.0;    self.matrix[2][1][0] = sx;   self.matrix[2][2][0] = 0.0;
		self.matrix[2][0][1] = sy;   self.matrix[2][1][1] = 1.0;    self.matrix[2][2][1] = 0.0;
		self.matrix[2][0][2] = 0.0;	  self.matrix[2][1][2] = 0.0;   self.matrix[2][2][2] = 1.0;
		self.multiply();
    }

    pub fn translate(&mut self, ox: f32, oy: f32) {
        // Construct Translate Matrix
		self.matrix[2][0][0] = 1.0;   self.matrix[2][1][0] = 0.0;   self.matrix[2][2][0] = ox;
		self.matrix[2][0][1] = 0.0;   self.matrix[2][1][1] = 1.0;   self.matrix[2][2][1] = oy;
		self.matrix[2][0][2] = 0.0;	  self.matrix[2][1][2] = 0.0;   self.matrix[2][2][2] = 1.0;
		self.multiply();
    }

    // TODO: not same API
    pub fn forward(&mut self, x: f32, y: f32) -> (f32, f32)
	{
		(x * self.matrix[self.source][0][0] + y * self.matrix[self.source][1][0] + self.matrix[self.source][2][0],
		x * self.matrix[self.source][0][1] + y * self.matrix[self.source][1][1] + self.matrix[self.source][2][1])
	}

    // TODO: not same API
    pub fn backward(&mut self, x: f32, y: f32) -> (f32, f32)
	{
		(x * self.matrix[3][0][0] + y * self.matrix[3][1][0] + self.matrix[3][2][0],
		x * self.matrix[3][0][1] + y * self.matrix[3][1][1] + self.matrix[3][2][1])
	}

    pub fn invert(&mut self)
	{
        // Obviously costly so only do if needed
		if self.dirty {			
			let det = self.matrix[self.source][0][0] * (self.matrix[self.source][1][1] * self.matrix[self.source][2][2] - self.matrix[self.source][1][2] * self.matrix[self.source][2][1]) -
				      self.matrix[self.source][1][0] * (self.matrix[self.source][0][1] * self.matrix[self.source][2][2] - self.matrix[self.source][2][1] * self.matrix[self.source][0][2]) +
				      self.matrix[self.source][2][0] * (self.matrix[self.source][0][1] * self.matrix[self.source][1][2] - self.matrix[self.source][1][1] * self.matrix[self.source][0][2]);

			let idet = 1.0 / det;
			self.matrix[3][0][0] = (self.matrix[self.source][1][1] * self.matrix[self.source][2][2] - self.matrix[self.source][1][2] * self.matrix[self.source][2][1]) * idet;
			self.matrix[3][1][0] = (self.matrix[self.source][2][0] * self.matrix[self.source][1][2] - self.matrix[self.source][1][0] * self.matrix[self.source][2][2]) * idet;
			self.matrix[3][2][0] = (self.matrix[self.source][1][0] * self.matrix[self.source][2][1] - self.matrix[self.source][2][0] * self.matrix[self.source][1][1]) * idet;
			self.matrix[3][0][1] = (self.matrix[self.source][2][1] * self.matrix[self.source][0][2] - self.matrix[self.source][0][1] * self.matrix[self.source][2][2]) * idet;
			self.matrix[3][1][1] = (self.matrix[self.source][0][0] * self.matrix[self.source][2][2] - self.matrix[self.source][2][0] * self.matrix[self.source][0][2]) * idet;
			self.matrix[3][2][1] = (self.matrix[self.source][0][1] * self.matrix[self.source][2][0] - self.matrix[self.source][0][0] * self.matrix[self.source][2][1]) * idet;
			self.matrix[3][0][2] = (self.matrix[self.source][0][1] * self.matrix[self.source][1][2] - self.matrix[self.source][0][2] * self.matrix[self.source][1][1]) * idet;
			self.matrix[3][1][2] = (self.matrix[self.source][0][2] * self.matrix[self.source][1][0] - self.matrix[self.source][0][0] * self.matrix[self.source][1][2]) * idet;
			self.matrix[3][2][2] = (self.matrix[self.source][0][0] * self.matrix[self.source][1][1] - self.matrix[self.source][0][1] * self.matrix[self.source][1][0]) * idet;
			self.dirty = false;
		}				
	}
}

pub struct GFX2D;

impl GFX2D {
    pub fn draw_sprite(pge: &mut PGE, sprite: &Sprite, transform: &mut Transform2D) {
        // Work out bounding rectangle of sprite
        let mut sx: f32 = 0.0;
        let mut sy: f32 = 0.0;
        let mut ex: f32 = 0.0;
        let mut ey: f32 = 0.0;
        let a = transform.forward(0.0, 0.0);
        sx = sx.min(a.0);  sy = sy.min(a.1);
        ex = ex.max(a.0);  ey = ey.max(a.1);
        let b = transform.forward(sprite.width as f32, sprite.height as f32);
        sx = sx.min(b.0);  sy = sy.min(b.1);
        ex = ex.max(b.0);  ey = ey.max(b.1);
        let c = transform.forward(0.0, sprite.height as f32);
        sx = sx.min(c.0);  sy = sy.min(c.1);
        ex = ex.max(c.0);  ey = ey.max(c.1);
        let d = transform.forward(sprite.width as f32, 0.0);
        sx = sx.min(d.0);  sy = sy.min(d.1);
        ex = ex.max(d.0);  ey = ey.max(d.1);

		// Perform inversion of transform if required
        transform.invert();

        if ex < sx {
            std::mem::swap(&mut ex, &mut sx);
        }
        if ey < sy {
            std::mem::swap(&mut ey, &mut sy);
        }

		// Iterate through render space, and sample Sprite from suitable texel location
        for i in sx as i32 .. ex as i32 {
            for j in sy as i32 .. ey as i32 {
                let o = transform.backward(i as f32, j as f32);
                pge.draw(i, j, &sprite.get_pixel((o.0 + 0.5) as i32, (o.1 + 0.5) as i32));
            }
        }
    }
}