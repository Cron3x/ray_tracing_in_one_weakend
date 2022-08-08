use std::ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign};

pub mod color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 (pub f64,pub f64,pub f64);

impl Add for Vec3 {
	type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
	type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Vec3 {
	type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
	type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Div<f64> for Vec3 {
	type Output = Self;

    fn div(self, other: f64) -> Self {
        Self(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
		self.1 += rhs.1; 
		self.2 += rhs.2;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
		self.1 *= rhs; 
		self.2 *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;