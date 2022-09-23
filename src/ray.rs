use geometric::Vector3;
type Point = Vector3<f64>;
pub struct Ray {
    pub origin: Point,
    pub dir: Vector3<f64>,
}

impl Ray {
    pub fn orig(&self) -> &Vector3<f64> {
        &self.origin
    }
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.dir * t
    }
    pub fn direction(&self) -> Vector3<f64> {
        self.dir.clone()
    }
}
