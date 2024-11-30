use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    x: i32,
    y: i32,
}

impl Complex {
    pub fn conjugate(&self) -> Complex {
        Complex {
            y: -self.y,
            ..*self // can populate remaining struct fields using a spread
        }
    }

    pub fn abs(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }

    pub fn new(x: i32, y: i32) -> Complex {
        Complex { x, y }
    }

    // can also define associated functions that don't take self
    pub fn origin() -> Complex {
        Complex { x: 0, y: 0 }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, right: Complex) -> Complex {
        Complex {
            x: self.x + right.x,
            y: self.y + right.y,
        }
    }
}

// For any T that implements adding an i32 to get an i32, implement Add for Complex
impl<T: Add<i32, Output=i32>> Add<T> for Complex {
    type Output = Complex;

    fn add(self, rhs: T) -> Self::Output {
        Complex {
            x: rhs + self.x,
            y: self.y
        }
    }
}
