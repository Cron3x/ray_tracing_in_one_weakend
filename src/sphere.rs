use crate::{vec3::{Point3, dot}, hittable::{Hittable, HitRecord}, ray::Ray};

pub struct Sphere{
    center:Point3,
    radius:f64,
}

impl Sphere {
    pub fn new(cen:Point3, r:f64) -> Self{
        Self{center:cen, radius:r}
    }
}

impl Hittable for Sphere{
    fn hit(self, r:&Ray, t_min:f64, t_max:f64, rec:&mut HitRecord) -> bool{
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius*self.radius;
        
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0f64 { return false }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b -sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;

        return true;
    }
}
