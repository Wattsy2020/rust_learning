#[derive(Debug)]
pub struct Complex {
    x: i32,
    y: i32,
}

impl Complex {
    pub fn add(&self, right: &Complex) -> Complex {
        Complex {
            x: self.x + right.x,
            y: self.y + right.y,
        }
    }

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
