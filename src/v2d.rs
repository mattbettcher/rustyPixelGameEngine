use num_traits::Float;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

// Very basic 2D vector type.
// This is a bit over complicated because I feature matched the C++ version javidx9 wrote.
//   Missing stuff - 
//      index operator
//      

#[derive(Debug, Clone)]
pub struct V2d<T> 
where 
    T : Float,
{
    pub x: T,
    pub y: T,
}

impl<T> V2d<T> where T : Float {
    pub fn new(x: T, y: T) -> Self {
        V2d { x, y }
    }

    pub fn mag(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn norm(&self) -> Self {
        let r = T::one() / self.mag();              // T::one() seems goofy, but this gives us the proper type for f32 or f64
        V2d { x: self.x * r, y: self.y * r }
    }

    pub fn perp(&self) -> Self {
        V2d { x: -self.y, y: self.x }
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn cross(&self, rhs: &Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T> Add for V2d<T> where T : Float {
    type Output = V2d<T>;

    fn add(self, rhs: V2d<T>) -> V2d<T> {
        V2d {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for V2d<T> where T : Float {
    type Output = V2d<T>;

    fn sub(self, rhs: V2d<T>) -> V2d<T> {
        V2d {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for V2d<T> where T : Float {
    type Output = V2d<T>;

    fn mul(self, rhs: T) -> V2d<T> {
        V2d {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for V2d<T> where T : Float {
    type Output = V2d<T>;

    fn div(self, rhs: T) -> V2d<T> {
        V2d {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> AddAssign for V2d<T> where T : Float {
    fn add_assign(&mut self, rhs: V2d<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> SubAssign for V2d<T> where T : Float {
    fn sub_assign(&mut self, rhs: V2d<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T> MulAssign<T> for V2d<T> where T : Float {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<T> DivAssign<T> for V2d<T> where T : Float {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
