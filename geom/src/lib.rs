use std::ops::{Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pos {
    pub x: i64,
    pub y: i64
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    pub x: i64,
    pub y: i64
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}

impl Default for Offset {
    fn default() -> Self {
        Offset { x: 0, y: 0 }
    }
}

impl Add<Offset> for Pos {
    type Output = Pos;
    fn add(self, offset: Offset) -> Self::Output {
        Pos { 
            x: self.x + offset.x,
            y: self.y + offset.y
        }
    }
}

impl Sub<Offset> for Pos {
    type Output = Pos;
    fn sub(self, offset: Offset) -> Self::Output {
        Pos { 
            x: self.x - offset.x,
            y: self.y - offset.y
        }
    }
}

impl Add<Offset> for Offset {
    type Output = Offset;
    fn add(self, offset: Offset) -> Self::Output {
        Offset { 
            x: self.x + offset.x,
            y: self.y + offset.y
        }
    }
}

impl Sub<Offset> for Offset {
    type Output = Offset;
    fn sub(self, offset: Offset) -> Self::Output {
        Offset { 
            x: self.x - offset.x,
            y: self.y - offset.y
        }
    }
}

impl AddAssign for Offset {
    fn add_assign(&mut self, offset: Offset) {
        self.x += offset.x;
        self.y += offset.y;
    }
}

impl SubAssign for Offset {
    fn sub_assign(&mut self, offset: Offset) {
        self.x -= offset.x;
        self.y -= offset.y;
    }
}

impl Mul<i64> for Offset {
    type Output = Offset;
    fn mul(self, scale: i64) -> Self::Output {
        Offset {
            x: self.x * scale,
            y: self.y * scale
        }
    }
}

impl MulAssign<i64> for Offset {
    fn mul_assign(&mut self, scale: i64) {
        self.x *= scale;
        self.y *= scale;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
