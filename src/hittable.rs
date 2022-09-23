use crate::ray::Ray;
use geometric::{Vector3, Dot};

//A Struct that stores the information about a hit
pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray) {
        let front_face = self.normal.dot(ray.direction())<0.0;
        self.normal = match front_face {
            true => self.normal,
            false => -self.normal,
        };
    }
} 

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
