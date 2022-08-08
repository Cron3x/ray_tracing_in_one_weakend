use crate::vec3::{Point3, Vec3};
trait Ray {
	fn ray();
	fn ray_args(origin: Point3, direction: Vec3);
	fn origin() -> Point3;
	fn direction() -> Vec3;
	fn at(t:f64) -> Point3 {Self::origin() + Self::direction() * t}
}