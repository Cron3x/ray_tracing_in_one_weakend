use crate::vec3_mod::{Point3, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
	pub orig:Point3,
	pub dir: Vec3,
}

impl Ray {
	pub fn ray(&mut self, origin: Point3, direction: Vec3){
		self.dir  = direction;
		self.orig = origin;
	}
	pub fn origin(self) -> Point3{
		self.orig
	}
	pub fn direction(self) -> Vec3{
		self.dir
	}
	pub fn at(self, t:f64) -> Point3 {(self.dir * t) + self.orig }
}