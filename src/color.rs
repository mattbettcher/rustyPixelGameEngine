use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// 0-1 rgb
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: r,
            g: g,
            b: b,
            a: 1.0,
        }
    }
}

/*
    r = new Color();
    r.A = 1 - (1 - fg.A) * (1 - bg.A);
    if (r.A < 1.0e-6) return r; // Fully transparent -- R,G,B not important
    r.R = fg.R * fg.A / r.A + bg.R * bg.A * (1 - fg.A) / r.A;
    r.G = fg.G * fg.A / r.A + bg.G * bg.A * (1 - fg.A) / r.A;
    r.B = fg.B * fg.A / r.A + bg.B * bg.A * (1 - fg.A) / r.A;
*/

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        let a = 1.0 - (1.0 - self.a) * (1.0 - rhs.a);
        Color {
            r: self.r * self.a / a + rhs.r * rhs.a * (1.0 - self.a) / a,
            g: self.g * self.a / a + rhs.g * rhs.a * (1.0 - self.a) / a,
            b: self.b * self.a / a + rhs.b * rhs.a * (1.0 - self.a) / a,
            a: a,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Color {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Color {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
        self.a = self.a + rhs.a;
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r = self.r - rhs.r;
        self.g = self.g - rhs.g;
        self.b = self.b - rhs.b;
        self.a = self.a - rhs.a;
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = self.r * rhs;
        self.g = self.g * rhs;
        self.b = self.b * rhs;
        self.a = self.a * rhs;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r = self.r / rhs;
        self.g = self.g / rhs;
        self.b = self.b / rhs;
        self.a = self.a / rhs;
    }
}
