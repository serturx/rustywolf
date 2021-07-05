use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        return Vector2 { x, y };
    }

    pub fn set(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

impl<T: Add> Add for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        return Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl<T: AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub> Sub for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        return Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl<T: SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        return Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        };
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        return Vector2 {
            x: -self.x,
            y: -self.y,
        };
    }
}
