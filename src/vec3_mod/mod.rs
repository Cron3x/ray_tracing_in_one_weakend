use std::{ops::{Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign}};

pub mod color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vec3 {
    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64{
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

pub fn unit_vector(v:Vec3) -> Vec3{
    v / v.length()
}
pub fn vec3(x:f64,y:f64,z:f64)->Vec3{
    Vec3 { x: x, y: y, z: z }
}

impl Add for Vec3 {
	type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3 {
	type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{x:self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul for Vec3 {
	type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self{x: self.x * other.x, y:self.y * other.y, z: self.z * other.z}
    }
}

impl Mul<f64> for Vec3 {
	type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self{x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl Div<f64> for Vec3 {
	type Output = Self;

    fn div(self, other: f64) -> Self {
        Self{x:self.x / other, y:self.y / other, z:self.z / other}
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
		self.y += rhs.y; 
		self.z += rhs.z;
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
		self.y *= rhs; 
		self.z *= rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;