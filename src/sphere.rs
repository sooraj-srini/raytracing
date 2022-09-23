
use geometric::Vector3;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use geometric::Dot;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.direction() - self.center;
        let a = ray.direction().mag_sq();
        let half_b = oc.dot(ray.direction());
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            false
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;

            if root < t_min || root > t_max {
                root = (-half_b + sqrtd) / a;
                if root < t_min || root > t_max {
                    return false;
                }
            }
            rec.t = root;
            rec.p = ray.at(root);
            rec.normal = (rec.p - self.center) / self.radius;
            rec.set_face_normal(ray);
            true
        }
    }
}
